use leptos::ev::SubmitEvent;
use leptos::leptos_dom::logging::console_log;
use leptos::{html, prelude::*};
use urlencoding::encode;

use crate::exercise::{ExerciseSummary, InputStyle};
use crate::utils::get_tones_from_pinyin;
use crate::{
    exercise::{ExerciseParams, ExerciseType, ShuangElement},
    utils::{
        format_url, get_pinyin_from_chinese_word, get_pronounced_pinyin,
        get_random_hanzi_pairs_idxs, get_tones_only_from_pronounced_pinyin,
    },
};

const DEFAULT_LISTENINGS_TRIES: u32 = 3;

/// A component handling the exercise session for ShuangShuang
#[component]
pub fn TestSession(
    hanzi_pairs: ReadSignal<Vec<String>>,
    exercise_params: ReadSignal<Option<ExerciseParams>>,
    set_exercise_finished: WriteSignal<bool>,
) -> impl IntoView {
    let (current_random_idx, set_current_random_idx) = signal(0);
    let (current_hanzi_pair, set_current_hanzi_pair) = signal("".to_string());
    let (user_answer, set_user_answer) = signal("".to_string());
    let (random_idxs, set_random_idxs) = signal(vec![]);
    let (shuang_elements, set_shuang_elements) = signal::<Vec<ShuangElement>>(vec![]);
    let (audio_playing, set_audio_playing) = signal(false);
    let (show_results, set_show_results) = signal(false);
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
        set_audio_url(format_url(&current_hanzi_pair(), params().audio_quality));
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
        ExerciseType::ToneOnly => "Enter which tones you hear",
        ExerciseType::NoTonePinyin => "Enter pinyin without tone marks",
        ExerciseType::Pinyin => "Enter full pinyin",
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
                answer = answer_elem.value();
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
        let pronounced_pinyin = get_pronounced_pinyin(
            &current_hanzi_pair(),
            &get_pinyin_from_chinese_word(&current_hanzi_pair()).unwrap(),
        );
        let tones = get_tones_from_pinyin(&pronounced_pinyin);
        let expected_answer = match params().exercise_type {
            ExerciseType::ToneOnly => get_tones_only_from_pronounced_pinyin(&pronounced_pinyin),
            ExerciseType::NoTonePinyin => pronounced_pinyin.replace(char::is_numeric, ""),
            ExerciseType::Pinyin => pronounced_pinyin.clone(),
        };
        set_shuang_elements.update(|v| {
            v.push(ShuangElement {
                characters: current_hanzi_pair(),
                is_correct: answer == expected_answer,
                pinyin: pronounced_pinyin,
                user_answer: answer,
                tone_combination: (tones[0], tones[1]),
            })
        });
        if current_random_idx() < random_idxs.read().len() - 1 {
            set_current_random_idx.update(|idx| *idx += 1);
            console_log(&format!("Current idx: {}", current_random_idx.read()));
            go_to_next_hanzi_pair();
            play_hanzi_pair_audio(true);
            set_remaining_listenings(DEFAULT_LISTENINGS_TRIES);
            console_log(&format!("Current url: {}", &audio_url()));
        } else {
            set_show_results(true);
        }
    };

    view! {
        {move || {
            if current_random_idx() == 0 {
                set_random_idxs(
                    get_random_hanzi_pairs_idxs(params().exercise_size, &hanzi_pairs()),
                );
                go_to_next_hanzi_pair();
            }
            if show_results() {
                let exercise_summary = ExerciseSummary::from(shuang_elements());
                view! {
                    <div>
                        <div>"Correct answers: " {exercise_summary.correct_answers}</div>
                        <div>
                            "Percentage correct: "
                            {format!("{:.2}%", exercise_summary.get_correct_percentage())}
                        </div>
                        <div>
                            {if exercise_summary.tone_combination_mistakes.len() > 0 {

                                view! {
                                    "Incorrect tone combinations: "
                                    {exercise_summary
                                        .tone_combination_mistakes
                                        .iter()
                                        .map(|(tone_combination, mistake_count)| {
                                            view! {
                                                <li>
                                                    {format!("{:?}: {mistake_count}", tone_combination)}
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
                                            "https://www.mdbg.net/chinese/dictionary?wdqb={}",
                                            encode(&elem.characters),
                                        );
                                        mistakes_views
                                            .push(

                                                view! {
                                                    <div>
                                                        <div>
                                                            <a href=elem_ref>{elem.characters.clone()}</a>
                                                        </div>
                                                        <div>"Expected pinyin answer: "{elem.pinyin.clone()}</div>
                                                        <div>
                                                            "Expected tone answer: "
                                                            {format!(
                                                                "{}{}",
                                                                elem.tone_combination.0.clone().to_string(),
                                                                elem.tone_combination.1.clone().to_string(),
                                                            )}
                                                        </div>
                                                        <div>"User answer: "{elem.user_answer.clone()}</div>
                                                    </div>
                                                }
                                                    .into_any(),
                                            );
                                    }
                                }
                                mistakes_views.collect_view()
                            }
                        </div>
                        <button on:click=move |_| {
                            set_exercise_finished(true)
                        }>"Go back to menu"</button>
                    </div>
                }
                    .into_any()
            } else {
                view! {
                    <div>
                        <div>
                            <form on:submit=move |ev: SubmitEvent| {
                                ev.prevent_default();
                                on_submit_answer()
                            }>
                                {if let InputStyle::Keyboard = params().input_style {
                                    view! {
                                        <label>
                                            {input_label}
                                            <input
                                                required
                                                type="text"
                                                node_ref=user_answer_element
                                                value=user_answer
                                            />
                                        </label>
                                        <input type="submit" value=">" />
                                    }
                                        .into_any()
                                } else {

                                    view! {
                                        <fieldset>
                                            <legend>"Select first tone value"</legend>
                                            {(1..6)
                                                .into_iter()
                                                .map(|tone| {
                                                    view! {
                                                        <label>
                                                            {tone}
                                                            <input
                                                                type="radio"
                                                                value=tone
                                                                bind:group=first_tone_value
                                                            />
                                                        </label>
                                                    }
                                                })
                                                .collect_view()}

                                        </fieldset>
                                        <fieldset>
                                            <legend>"Select second tone value"</legend>
                                            {(1..6)
                                                .into_iter()
                                                .map(|tone| {
                                                    view! {
                                                        <label>
                                                            {tone}
                                                            <input
                                                                type="radio"
                                                                value=tone
                                                                bind:group=second_tone_value
                                                            />
                                                        </label>
                                                    }
                                                })
                                                .collect_view()}

                                        </fieldset>
                                        <input type="submit" value="Ok" />
                                    }
                                        .into_any()
                                }}

                            </form>
                        </div>
                        <div>
                            <label>
                                "Remaining listenings: " {remaining_listenings}
                                <audio
                                    autoplay
                                    node_ref=audio_element
                                    on:ended=move |_| { set_audio_playing(false) }
                                >
                                    <source type="audio/mpeg" src=audio_url />
                                </audio>
                            </label>
                            {move || {
                                if audio_playing() || *remaining_listenings.read() == 0 {

                                    view! { <button disabled>"Play audio"</button> }
                                        .into_any()
                                } else {

                                    view! { <button on:click=on_click_audio>"Play audio"</button> }
                                        .into_any()
                                }
                            }}
                        </div>
                    </div>
                }
                    .into_any()
            }
        }}
    }
}
