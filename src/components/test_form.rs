use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::prelude::*;

use std::str::FromStr;

use crate::components::subtitle_with_helper::SubtitleWithHelper;
use crate::exercise::{
    AudioQuality, ExerciseDifficulty, ExerciseParams, ExerciseType, InputStyle, ShuffleMode,
};
use crate::i18n::*;

/// A component to enter exercise parameters for ShuangShuang
#[component]
pub fn TestForm(set_exercise_params: WriteSignal<Option<ExerciseParams>>) -> impl IntoView {
    let i18n = use_i18n();
    let (nb_elements, set_nb_elements) = signal(5);
    let exercise_difficulty = RwSignal::new(ExerciseDifficulty::default().to_string());
    let shuffle_mode = RwSignal::new(ShuffleMode::default().to_string());
    let exercise_type = RwSignal::new(ExerciseType::default().to_string());
    let input_style = RwSignal::new(InputStyle::default().to_string());
    let timer_on = RwSignal::new(false);
    let audio_quality = RwSignal::new(AudioQuality::default().to_string());
    let nb_elements_element: NodeRef<html::Input> = NodeRef::new();
    let nb_elements_min = 5;
    let nb_elements_max = 40;
    let nb_elements_step = 5;
    let nb_audio_retries = 3;

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let difficulty = ExerciseDifficulty::from_str(&exercise_difficulty()).unwrap();
        if let ExerciseDifficulty::Custom = difficulty {
            let exercise_size: u32 = nb_elements_element
                .get()
                .expect("<nb_elements_element> input should be mounted.")
                .value()
                .parse()
                .unwrap();
            set_nb_elements(exercise_size);
            set_exercise_params(Some(ExerciseParams {
                exercise_size,
                exercise_type: ExerciseType::from_str(&exercise_type()).unwrap(),
                input_style: InputStyle::from_str(&input_style()).unwrap(),
                shuffle_mode: ShuffleMode::from_str(&shuffle_mode()).unwrap(),
                timer_on: timer_on(),
                audio_quality: AudioQuality::from_str(&audio_quality()).unwrap(),
                audio_retries: nb_audio_retries,
            }));
        } else {
            let exercise_params = ExerciseParams::from(difficulty);
            set_exercise_params(Some(exercise_params));
        }
    };
    let fieldset_class = "flex flex-wrap justify-center py-2";
    let radio_class = "radio radio-sm radio-primary";
    let label_class = "p-2";
    let subtitle_font_class = "rounded-sm bg-neutral underline px-1";
    let hidden_class = move || {
        if let ExerciseDifficulty::Custom =
            ExerciseDifficulty::from_str(&exercise_difficulty()).unwrap()
        {
            ""
        } else {
            "hidden"
        }
    };
    let difficulty_helper_desc = {
        let i18n = i18n.clone();
        move || {
            let mut difficulty_helper_desc_view: Vec<AnyView> = vec![];
            difficulty_helper_desc_view.push(
                view! { {t!(i18n, form.select_difficulty_helper_desc_ft, <b> = <li />)} }
                    .into_any(),
            );
            difficulty_helper_desc_view.push(
                view! { {t!(i18n, form.select_difficulty_helper_desc_easy, <b> = <li />)} }
                    .into_any(),
            );
            difficulty_helper_desc_view.push(
                view! { {t!(i18n, form.select_difficulty_helper_desc_normal, <b> = <li />)} }
                    .into_any(),
            );
            difficulty_helper_desc_view.push(
                view! { {t!(i18n, form.select_difficulty_helper_desc_hard, <b> = <li />)} }
                    .into_any(),
            );
            difficulty_helper_desc_view.push(
                view! { {t!(i18n, form.select_difficulty_helper_desc_native, <b> = <li />)} }
                    .into_any(),
            );
            difficulty_helper_desc_view.push(
                view! { {t!(i18n, form.select_difficulty_helper_desc_custom, <b> = <li />)} }
                    .into_any(),
            );
            difficulty_helper_desc_view.collect_view()
        }
    };
    let shuffle_mode_helper_desc = {
        let i18n = i18n.clone();
        move || {
            let mut shuffle_mode_helper_desc_view: Vec<AnyView> = vec![];
            shuffle_mode_helper_desc_view.push(
                view! { {t!(i18n, form.select_shuffle_mode_helper_desc_evenly, <b> = <li />)} }
                    .into_any(),
            );
            shuffle_mode_helper_desc_view.push(
                view! { {t!(i18n, form.select_shuffle_mode_helper_desc_random, <b> = <li />)} }
                    .into_any(),
            );
            shuffle_mode_helper_desc_view.collect_view()
        }
    };

    view! {
        {move || {
            view! {
                <div class="flex justify-center">
                    <div class="flex card bg-base-100 card-border border-base-300 card-md overflow-hidden px-10 max-w-[450px]">
                        <form on:submit=on_submit>
                            <div class="py-2">
                                <fieldset>
                                    <SubtitleWithHelper
                                        subtitle=t_string!(i18n, form.select_difficulty).to_string()
                                        helper_title=t_string!(i18n, form.select_difficulty_helper)
                                            .to_string()
                                        helper_desc=move || {
                                            view! { {difficulty_helper_desc()} }
                                        }
                                    />
                                    <div class=fieldset_class>
                                        <label class=label_class>
                                            {t!(i18n, form.first_time_difficulty)}
                                            <input
                                                type="radio"
                                                class=radio_class
                                                value=ExerciseDifficulty::FirstTime.to_string()
                                                bind:group=exercise_difficulty
                                            />
                                        </label>
                                        <label class=label_class>
                                            {t!(i18n, form.easy_difficulty)}
                                            <input
                                                type="radio"
                                                class=radio_class
                                                value=ExerciseDifficulty::Easy.to_string()
                                                bind:group=exercise_difficulty
                                            />
                                        </label>
                                        <label class=label_class>
                                            {t!(i18n, form.normal_difficulty)}
                                            <input
                                                type="radio"
                                                class=radio_class
                                                value=ExerciseDifficulty::Normal.to_string()
                                                bind:group=exercise_difficulty
                                            />
                                        </label>
                                        <label class=label_class>
                                            {t!(i18n, form.hard_difficulty)}
                                            <input
                                                type="radio"
                                                class=radio_class
                                                value=ExerciseDifficulty::Hard.to_string()
                                                bind:group=exercise_difficulty
                                            />
                                        </label>
                                        <label class=label_class>
                                            {t!(i18n, form.native_difficulty)}
                                            <input
                                                type="radio"
                                                class=radio_class
                                                value=ExerciseDifficulty::Native.to_string()
                                                bind:group=exercise_difficulty
                                            />
                                        </label>
                                        <label class=label_class>
                                            {t!(i18n, form.custom_difficulty)}
                                            <input
                                                type="radio"
                                                class=radio_class
                                                value=ExerciseDifficulty::Custom.to_string()
                                                bind:group=exercise_difficulty
                                            />
                                        </label>
                                    </div>
                                </fieldset>
                            </div>
                            <div class=hidden_class>
                                <div class="py-2">
                                    <label for="nb_elements" class=subtitle_font_class>
                                        {t!(i18n, form.number_of_test_questions)}
                                    </label>
                                    <div class="w-full justify-items-center">
                                        <input
                                            type="range"
                                            class="w-full range range-sm range-primary [--range-thumb:white]"
                                            min=nb_elements_min
                                            max=nb_elements_max
                                            step=nb_elements_step
                                            value=nb_elements
                                            node_ref=nb_elements_element
                                            list="values"
                                        />
                                        {move || {
                                            let range_labels_class = "flex justify-between px-2.5 mt-1 text-xs";
                                            let mut range_ticks_view: Vec<AnyView> = vec![];
                                            let mut range_labels_view: Vec<AnyView> = vec![];
                                            for element_step in (nb_elements_min..nb_elements_max + 1)
                                                .step_by(nb_elements_step)
                                            {
                                                range_ticks_view
                                                    .push(view! { <span>"|"</span> }.into_any());
                                                range_labels_view
                                                    .push(view! { <span>{element_step}</span> }.into_any());
                                            }

                                            view! {
                                                <div class=range_labels_class>
                                                    {range_ticks_view.collect_view()}
                                                </div>
                                                <div class=range_labels_class>
                                                    {range_labels_view.collect_view()}
                                                </div>
                                            }
                                        }}

                                    </div>

                                </div>
                                <div>
                                    <fieldset>
                                        <legend class=subtitle_font_class>
                                            {t!(i18n, form.select_exercise_type)}
                                        </legend>
                                        <div class=fieldset_class>
                                            <label class=label_class>
                                                {t!(i18n, form.exercise_type_tone_only)}
                                                <input
                                                    type="radio"
                                                    class=radio_class
                                                    value=ExerciseType::ToneOnly.to_string()
                                                    bind:group=exercise_type
                                                />
                                            </label>
                                            <label class=label_class>
                                                {t!(i18n, form.exercise_type_pinyin)}
                                                <input
                                                    type="radio"
                                                    class=radio_class
                                                    value=ExerciseType::Pinyin.to_string()
                                                    bind:group=exercise_type
                                                />
                                            </label>
                                        </div>
                                    </fieldset>
                                </div>
                                <div>
                                    <fieldset>
                                        <legend class=subtitle_font_class>
                                            {t!(i18n, form.select_input_type)}
                                        </legend>
                                        <div class=fieldset_class>

                                            {move || {
                                                if let ExerciseType::ToneOnly = ExerciseType::from_str(
                                                        &exercise_type(),
                                                    )
                                                    .unwrap()
                                                {
                                                    view! {
                                                        <label class=label_class>
                                                            {t!(i18n, form.input_type_keyboard)}
                                                            <input
                                                                type="radio"
                                                                class=radio_class
                                                                value=InputStyle::Keyboard.to_string()
                                                                bind:group=input_style
                                                            />
                                                        </label>
                                                        <label class=label_class>
                                                            {t!(i18n, form.input_type_buttons)}
                                                            <input
                                                                type="radio"
                                                                class=radio_class
                                                                value=InputStyle::Touch.to_string()
                                                                bind:group=input_style
                                                            />
                                                        </label>
                                                    }
                                                        .into_any()
                                                } else {
                                                    input_style.set(InputStyle::Keyboard.to_string());
                                                    view! {
                                                        <label class=label_class>
                                                            {t!(i18n, form.input_type_keyboard)}
                                                            <input
                                                                disabled
                                                                type="radio"
                                                                class=radio_class
                                                                value=InputStyle::Keyboard.to_string()
                                                                bind:group=input_style
                                                            />
                                                        </label>
                                                        <label class=label_class>
                                                            {t!(i18n, form.input_type_buttons)}
                                                            <input
                                                                disabled
                                                                type="radio"
                                                                class=radio_class
                                                                value=InputStyle::Touch.to_string()
                                                                bind:group=input_style
                                                            />
                                                        </label>
                                                    }
                                                        .into_any()
                                                }
                                            }}
                                        </div>
                                    </fieldset>
                                </div>
                                <div>
                                    <fieldset>
                                        <SubtitleWithHelper
                                            subtitle=t_string!(i18n, form.select_shuffle_mode)
                                                .to_string()
                                            helper_title=t_string!(
                                                i18n, form.select_shuffle_mode_helper
                                            )
                                                .to_string()
                                            helper_desc=move || {
                                                view! { {shuffle_mode_helper_desc()} }
                                            }
                                        />
                                        <div class=fieldset_class>
                                            <label class=label_class>
                                                {t!(i18n, form.shuffle_mode_evenly)}
                                                <input
                                                    type="radio"
                                                    class=radio_class
                                                    value=ShuffleMode::Even.to_string()
                                                    bind:group=shuffle_mode
                                                />
                                            </label>
                                            <label class=label_class>
                                                {t!(i18n, form.shuffle_mode_random)}
                                                <input
                                                    type="radio"
                                                    class=radio_class
                                                    value=ShuffleMode::Random.to_string()
                                                    bind:group=shuffle_mode
                                                />
                                            </label>
                                        </div>
                                    </fieldset>
                                </div>
                                <div>
                                    <fieldset>
                                        <legend class=subtitle_font_class>
                                            {t!(i18n, form.select_audio_quality)}
                                        </legend>
                                        <div class=fieldset_class>
                                            <label class=label_class>
                                                {t!(i18n, form.low_audio_quality)}
                                                <input
                                                    type="radio"
                                                    class=radio_class
                                                    value=AudioQuality::Q18k.to_string()
                                                    bind:group=audio_quality
                                                />
                                            </label>
                                            <label class=label_class>
                                                {t!(i18n, form.medium_audio_quality)}
                                                <input
                                                    type="radio"
                                                    class=radio_class
                                                    value=AudioQuality::Q24k.to_string()
                                                    bind:group=audio_quality
                                                />
                                            </label>
                                            <label class=label_class>
                                                {t!(i18n, form.high_audio_quality)}
                                                <input
                                                    type="radio"
                                                    class=radio_class
                                                    value=AudioQuality::Q64k.to_string()
                                                    bind:group=audio_quality
                                                />
                                            </label>
                                            <label class=label_class>
                                                {t!(i18n, form.best_audio_quality)}
                                                <input
                                                    type="radio"
                                                    class=radio_class
                                                    value=AudioQuality::Q96k.to_string()
                                                    bind:group=audio_quality
                                                />
                                            </label>
                                        </div>
                                    </fieldset>
                                </div>
                                <div>
                                    <fieldset>
                                        <legend class=subtitle_font_class>
                                            {t!(i18n, form.toggle_timer_option)}
                                        </legend>
                                        <div class=fieldset_class>
                                            <input
                                                type="checkbox"
                                                class="toggle toggle-primary toggle-md"
                                                bind:checked=timer_on
                                            />
                                        </div>
                                    </fieldset>
                                </div>
                            </div>
                            <div class="flex justify-center m-2 pb-2">
                                <input
                                    type="submit"
                                    class="btn btn-accent text-white rounded-md"
                                    value=t_string!(i18n, form.start_practice_btn)
                                />
                            </div>
                        </form>
                    </div>
                </div>
            }
        }}
    }
}
