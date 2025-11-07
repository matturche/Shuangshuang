use leptos::ev::SubmitEvent;
use leptos::leptos_dom::logging::console_log;
use leptos::{html, prelude::*};
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;
use urlencoding::encode;

use crate::exercise::{ExerciseSummary, HanziPair, InputStyle};
use crate::i18n::*;
use crate::{
    exercise::{ExerciseParams, ExerciseType, ShuangElement},
    utils::{format_url, get_random_hanzi_pairs_idxs, get_tones_only_from_pronounced_pinyin},
};

const DEFAULT_LISTENINGS_TRIES: u32 = 3;
const DEFAULT_TIMER_VALUE: u32 = 5;

/// A component handling the exercise session for ShuangShuang
#[component]
pub fn TestSession(
    hanzi_pairs: ReadSignal<Vec<HanziPair>>,
    exercise_params: ReadSignal<Option<ExerciseParams>>,
    set_exercise_finished: WriteSignal<bool>,
) -> impl IntoView {
    let i18n = use_i18n();
    let (current_random_idx, set_current_random_idx) = signal(0);
    let (current_hanzi_pair, set_current_hanzi_pair) = signal(HanziPair::default());
    let (user_answer, set_user_answer) = signal("".to_string());
    let (random_idxs, set_random_idxs) = signal(vec![]);
    let (shuang_elements, set_shuang_elements) = signal::<Vec<ShuangElement>>(vec![]);
    let (audio_playing, set_audio_playing) = signal(false);
    let (show_results, set_show_results) = signal(false);
    let timer = RwSignal::new(DEFAULT_TIMER_VALUE);
    let first_tone_value = RwSignal::new("".to_string());
    let second_tone_value = RwSignal::new("".to_string());
    let user_answer_element: NodeRef<html::Input> = NodeRef::new();
    let audio_element: NodeRef<html::Audio> = NodeRef::new();
    let (remaining_listenings, set_remaining_listenings) = signal(DEFAULT_LISTENINGS_TRIES);
    let (audio_url, set_audio_url) = signal("".to_string());
    let params = move || exercise_params().expect("exercise_params is not yet set to Some.");
    let go_to_next_hanzi_pair = move || {
        let c_r_idx: usize = current_random_idx();
        let c_idx: usize = random_idxs.read()[c_r_idx];
        set_current_hanzi_pair(hanzi_pairs.read()[c_idx].clone());
        set_audio_url(format_url(
            &current_hanzi_pair().characters,
            params().audio_quality,
        ));
        set_remaining_listenings(params().audio_retries);
        console_log(&format!("Current idx: {}", c_r_idx));
        console_log(&format!("Current url: {}", audio_url()));
    };
    let play_hanzi_pair_audio = move |set_new_audio: bool| {
        let audio = audio_element
            .get()
            .expect("<audio> element should be mounted");
        if set_new_audio {
            audio.set_src(&audio_url());
        }
        let _promise = audio.play().expect("Failed to play audio element.");
        set_audio_playing(true);
    };

    let input_label = move || match params().exercise_type {
        ExerciseType::ToneOnly => t_string!(i18n, exercise.input_label_tone_only),
        ExerciseType::NoTonePinyin => t_string!(i18n, exercise.input_label_no_tone_pinyin),
        ExerciseType::Pinyin => t_string!(i18n, exercise.input_label_pinyin),
    };

    let on_click_audio = move |_| {
        if *remaining_listenings.read() > 0 {
            set_remaining_listenings.update(|l| *l -= 1);
            play_hanzi_pair_audio(false);
        }
    };

    let on_submit_answer = move || {
        let answer: String;
        match params().input_style {
            InputStyle::Keyboard => {
                let answer_elem = user_answer_element
                    .get()
                    .expect("<user_answer_element> should be mounted.");
                answer = answer_elem.value().to_lowercase();
                // Clean the value inside the input to prepare for next question
                answer_elem.set_value("");
                set_user_answer(answer.clone());
            }
            InputStyle::Touch => {
                answer = first_tone_value() + &second_tone_value();
                first_tone_value.set("".to_string());
                second_tone_value.set("".to_string());
            }
        }
        let hanzi_pair = current_hanzi_pair();
        let pronounced_pinyin = hanzi_pair.pronounced_pinyin.clone();
        let expected_answer = match params().exercise_type {
            ExerciseType::ToneOnly => get_tones_only_from_pronounced_pinyin(&pronounced_pinyin),
            ExerciseType::NoTonePinyin => pronounced_pinyin.replace(char::is_numeric, ""),
            ExerciseType::Pinyin => pronounced_pinyin.clone(),
        };
        set_shuang_elements.update(|v| {
            v.push(ShuangElement {
                hanzi_pair,
                is_correct: answer == expected_answer,
                user_answer: answer,
            })
        });
        if current_random_idx() < random_idxs.read().len() - 1 {
            set_current_random_idx.update(|idx| *idx += 1);
            go_to_next_hanzi_pair();
            play_hanzi_pair_audio(true);
            if params().timer_on {
                timer.set(DEFAULT_TIMER_VALUE);
            }
        } else {
            set_show_results(true);
        }
    };

    // Function to handle countdown
    let Pausable {
        pause,
        resume,
        is_active,
    } = use_interval_fn(
        move || {
            if timer.get() > 0 {
                timer.update(|t| *t -= 1);
            } else {
                on_submit_answer();
                timer.set(DEFAULT_TIMER_VALUE);
            }
        },
        1000,
    );
    view! {
        {move || {
            if current_random_idx() == 0 && timer() == DEFAULT_TIMER_VALUE {
                pause();
                if params().timer_on {
                    resume();
                }
                set_random_idxs(
                    get_random_hanzi_pairs_idxs(params().exercise_size, &hanzi_pairs()),
                );
                go_to_next_hanzi_pair();
            }
            if show_results() {
                let exercise_summary = ExerciseSummary::from(shuang_elements());
                if is_active() {
                    pause();
                }

                view! {
                    <div class="flex justify-center">
                        <div class="flex flex-col">
                            <div class="card h-full md:h-160 md:mt-10 bg-base-100 card-border border-base-300 card-md overflow-auto px-10">
                                <div class="flex justify-center text-success">
                                    {t!(i18n, exercise.correct_answers)}
                                    {exercise_summary.correct_answers}
                                </div>
                                <div class="flex justify-center text-success">
                                    {t!(i18n, exercise.correct_percentage)}
                                    {format!("{:.2}%", exercise_summary.get_correct_percentage())}
                                </div>
                                <div class="flex flex-col justify-center text-center text-error">
                                    {if exercise_summary.tone_pair_mistakes.len() > 0 {

                                        view! {
                                            {t!(i18n, exercise.incorrect_tone_pairs)}
                                            {exercise_summary
                                                .tone_pair_mistakes
                                                .iter()
                                                .map(|(tone_pair, mistake_count)| {
                                                    view! {
                                                        <li>
                                                            {format!(
                                                                "({}, {}) => {mistake_count}",
                                                                tone_pair.0.to_string(),
                                                                tone_pair.1.to_string(),
                                                            )}
                                                        </li>
                                                    }
                                                })
                                                .collect_view()}
                                        }
                                            .into_any()
                                    } else {
                                        view! {}.into_any()
                                    }}

                                </div>
                                <div>
                                    {
                                        let mut mistakes_views: Vec<AnyView> = vec![];
                                        for elem in shuang_elements.read().iter() {
                                            if !elem.is_correct {
                                                let elem_ref = format!(
                                                    "{}{}",
                                                    t_string!(i18n, exercise.dictionnary_link),
                                                    encode(&elem.hanzi_pair.characters),
                                                );
                                                mistakes_views
                                                    .push(

                                                        view! {
                                                            <div class="flex flex-col justify-center">
                                                                <div>
                                                                    <a class="link link-info" href=elem_ref>
                                                                        {elem.hanzi_pair.characters.clone()}
                                                                    </a>
                                                                </div>
                                                                <div>
                                                                    {t!(i18n, exercise.expected_pinyin_answer)}
                                                                    {elem.hanzi_pair.pronounced_pinyin.clone()}
                                                                </div>
                                                                <div>
                                                                    {t!(i18n, exercise.expected_tone_answer)}
                                                                    {format!(
                                                                        "{}{}",
                                                                        elem.hanzi_pair.pronounced_tone_pair.0.clone().to_string(),
                                                                        elem.hanzi_pair.pronounced_tone_pair.1.clone().to_string(),
                                                                    )}
                                                                </div>
                                                                <div>
                                                                    {t!(i18n, exercise.user_answer)}{elem.user_answer.clone()}
                                                                </div>
                                                            </div>
                                                        }
                                                            .into_any(),
                                                    );
                                            }
                                        }
                                        mistakes_views.collect_view()
                                    }
                                </div>
                            </div>
                            <div class="flex fit justify-center py-2">
                                <button
                                    class="btn rounded-md btn-secondary text-white"
                                    on:click=move |_| { set_exercise_finished(true) }
                                >
                                    {t!(i18n, exercise.return_home)}
                                </button>
                            </div>
                        </div>
                    </div>
                }
                    .into_any()
            } else {
                let countdown_style = move || { format!("--value:{}", timer.get()) };
                let countdown_class = move || match timer() {
                    0..2 => "text-error",
                    2..4 => "text-warning",
                    _ => "text-success",
                };
                view! {
                    <div class="flex h-full md:h-100 justify-center place-items-center">
                        <div class="flex flex-col justify-center">
                            <div>
                                <a class="badge badge-accent text-white font-semibold">
                                    {random_idxs.read().len() - current_random_idx()}
                                </a>
                                {t!(i18n, exercise.remaining_pairs)}
                            </div>
                            {if params().timer_on {
                                view! {
                                    <div>
                                        {t!(i18n, exercise.remaining_time)}
                                        <span class="countdown font-mono">
                                            <span
                                                style=countdown_style()
                                                aria-live="polite"
                                                class=countdown_class()
                                            >
                                                {move || timer.get()}
                                            </span>
                                        </span>
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! {}.into_any()
                            }}

                            <div class="pt-2">
                                <form on:submit={
                                    let pause = pause.clone();
                                    let resume = resume.clone();
                                    let timer_on = params().timer_on;
                                    move |ev: SubmitEvent| {
                                        ev.prevent_default();
                                        if timer_on {
                                            pause();
                                        }
                                        on_submit_answer();
                                        if timer_on {
                                            resume();
                                        }
                                    }
                                }>
                                    {if let InputStyle::Keyboard = params().input_style {
                                        let input_placeholder: &str;
                                        let input_help: &str;
                                        let input_type: &str;
                                        match params().exercise_type {
                                            ExerciseType::Pinyin => {
                                                input_help = &t_string!(i18n, exercise.input_help_pinyin);
                                                input_placeholder = &t_string!(
                                                    i18n, exercise.input_placeholder_pinyin
                                                );
                                                input_type = "text";
                                            }
                                            ExerciseType::ToneOnly => {
                                                input_help = &t_string!(
                                                    i18n, exercise.input_help_tone_only
                                                );
                                                input_placeholder = &t_string!(
                                                    i18n, exercise.input_placeholder_tone_only
                                                );
                                                input_type = "number";
                                            }
                                            ExerciseType::NoTonePinyin => {
                                                input_help = &t_string!(
                                                    i18n, exercise.input_help_no_tone_pinyin
                                                );
                                                input_placeholder = &t_string!(
                                                    i18n, exercise.input_placeholder_no_tone_pinyin
                                                );
                                                input_type = "text";
                                            }
                                        }
                                        view! {
                                            <fieldset class="fieldset">
                                                <legend class="w-full text-xs font-semibold">
                                                    {input_label}
                                                </legend>
                                                <div class="flex fit">
                                                    <input
                                                        class="input input-neutral rounded-md text-[16px]"
                                                        required
                                                        autocapitalize="none"
                                                        type=input_type
                                                        node_ref=user_answer_element
                                                        value=user_answer
                                                        placeholder=input_placeholder
                                                    />
                                                    <input
                                                        class="btn btn-primary text-white rounded-sm"
                                                        type="submit"
                                                        value=">"
                                                    />
                                                </div>
                                                <p class="label">{input_help}</p>
                                            </fieldset>
                                        }
                                            .into_any()
                                    } else {
                                        let radio_class = "radio radio-sm radio-primary";
                                        let radio_space_class = "px-1";

                                        view! {
                                            <div class="flex flex-row justify-center mb-6 mt-2">
                                                <div>
                                                    <fieldset>
                                                        <legend class="fieldset-legend">
                                                            {t!(i18n, exercise.select_first_tone_value)}
                                                        </legend>
                                                        {(1..5)
                                                            .into_iter()
                                                            .map(|tone| {
                                                                view! {
                                                                    <label class=radio_space_class>
                                                                        {tone}
                                                                        <input
                                                                            type="radio"
                                                                            class=radio_class
                                                                            value=tone
                                                                            bind:group=first_tone_value
                                                                            required
                                                                        />
                                                                    </label>
                                                                }
                                                            })
                                                            .collect_view()}

                                                    </fieldset>
                                                    <fieldset>
                                                        <legend class="fieldset-legend">
                                                            {t!(i18n, exercise.select_second_tone_value)}
                                                        </legend>
                                                        {(1..6)
                                                            .into_iter()
                                                            .map(|tone| {
                                                                view! {
                                                                    <label class=radio_space_class>
                                                                        {tone}
                                                                        <input
                                                                            type="radio"
                                                                            class=radio_class
                                                                            value=tone
                                                                            bind:group=second_tone_value
                                                                            required
                                                                        />
                                                                    </label>
                                                                }
                                                            })
                                                            .collect_view()}

                                                    </fieldset>
                                                </div>
                                                <div class="flex justify-center place-items-center ml-4">
                                                    <input
                                                        class="btn btn-primary text-white rounded-md"
                                                        type="submit"
                                                        value="Ok"
                                                    />
                                                </div>
                                            </div>
                                        }
                                            .into_any()
                                    }}

                                </form>
                            </div>
                            <div class="flex flex-wrap">
                                <label class="label">
                                    {t!(i18n, exercise.remaining_listenings)} {remaining_listenings}
                                    <audio
                                        autoplay
                                        node_ref=audio_element
                                        on:ended=move |_| { set_audio_playing(false) }
                                    >
                                        <source type="audio/mpeg" src=audio_url />
                                    </audio>
                                </label>
                                {move || {
                                    let btn_class = "btn btn-neutral rounded-md mx-2";
                                    if audio_playing() || *remaining_listenings.read() == 0 {

                                        view! {
                                            <button class=btn_class disabled>
                                                {t!(i18n, exercise.replay_audio)}
                                            </button>
                                        }
                                            .into_any()
                                    } else {

                                        view! {
                                            <button class=btn_class on:click=on_click_audio>
                                                {t!(i18n, exercise.replay_audio)}
                                            </button>
                                        }
                                            .into_any()
                                    }
                                }}
                            </div>
                            <div class="flex justify-center">
                                <button
                                    class="link text-xs mt-6"
                                    on:click=move |_| { set_exercise_finished(true) }
                                >
                                    {t!(i18n, exercise.return_home)}
                                </button>
                            </div>
                        </div>
                    </div>
                }
                    .into_any()
            }
        }}
    }
}
