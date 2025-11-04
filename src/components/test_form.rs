use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::prelude::*;

use std::str::FromStr;

use crate::exercise::AudioQuality;
use crate::exercise::ExerciseParams;
use crate::exercise::ExerciseType;
use crate::exercise::InputStyle;

/// A component to enter exercise parameters for ShuangShuang
#[component]
pub fn TestForm(set_exercise_params: WriteSignal<Option<ExerciseParams>>) -> impl IntoView {
    let (nb_elements, set_nb_elements) = signal(5);
    let exercise_type = RwSignal::new(ExerciseType::default().to_string());
    let input_style = RwSignal::new(InputStyle::default().to_string());
    // let (timer_on, set_timer_on) = signal(false);
    let audio_quality = RwSignal::new(AudioQuality::default().to_string());
    let nb_elements_element: NodeRef<html::Input> = NodeRef::new();
    let nb_elements_min = 5;
    let nb_elements_max = 30;
    let nb_elements_step = 5;
    // let timer_on_element: NodeRef<html::Input> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
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
            timer_on: false,
            audio_quality: AudioQuality::from_str(&audio_quality()).unwrap(),
        }));
    };
    let fieldset_class = "flex justify-center py-2";
    let radio_class = "radio radio-sm radio-primary";
    let label_class = "px-2";
    let subtitle_font_class = "rounded-sm bg-neutral underline px-1";
    view! {
        <div class="flex justify-center">
            <div class="card bg-base-100 card-border border-base-300 card-md overflow-hidden px-10">
                <form on:submit=on_submit>
                    <div class="py-2">
                        <label for="nb_elements" class=subtitle_font_class>
                            "Number of test questions"
                        </label>
                        <div class="w-full max-w-xs">
                            <input
                                type="range"
                                class="range range-sm range-primary [--range-thumb:white]"
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
                                    range_ticks_view.push(view! { <span>"|"</span> }.into_any());
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
                            <legend class=subtitle_font_class>"Select exercise type"</legend>
                            <div class=fieldset_class>
                                <label class=label_class>
                                    <a>"Tone Only"</a>
                                    <input
                                        type="radio"
                                        class=radio_class
                                        value=ExerciseType::ToneOnly.to_string()
                                        bind:group=exercise_type
                                    />
                                </label>
                                <label class=label_class>
                                    "Pinyin"
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
                            <legend class=subtitle_font_class>"Select input style"</legend>
                            <div class=fieldset_class>

                                {move || {
                                    if let ExerciseType::ToneOnly = ExerciseType::from_str(
                                            &exercise_type(),
                                        )
                                        .unwrap()
                                    {
                                        view! {
                                            <label class=label_class>
                                                "Keyboard"
                                                <input
                                                    type="radio"
                                                    class=radio_class
                                                    value=InputStyle::Keyboard.to_string()
                                                    bind:group=input_style
                                                />
                                            </label>
                                            <label class=label_class>
                                                "Buttons"
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
                                                "Keyboard"
                                                <input
                                                    disabled
                                                    type="radio"
                                                    class=radio_class
                                                    value=InputStyle::Keyboard.to_string()
                                                    bind:group=input_style
                                                />
                                            </label>
                                            <label class=label_class>
                                                "Buttons"
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
                            <legend class=subtitle_font_class>"Select audio quality"</legend>
                            <div class=fieldset_class>
                                <label class=label_class>
                                    "Low"
                                    <input
                                        type="radio"
                                        class=radio_class
                                        value=AudioQuality::Q18k.to_string()
                                        bind:group=audio_quality
                                    />
                                </label>
                                <label class=label_class>
                                    "Medium"
                                    <input
                                        type="radio"
                                        class=radio_class
                                        value=AudioQuality::Q24k.to_string()
                                        bind:group=audio_quality
                                    />
                                </label>
                                <label class=label_class>
                                    "High"
                                    <input
                                        type="radio"
                                        class=radio_class
                                        value=AudioQuality::Q64k.to_string()
                                        bind:group=audio_quality
                                    />
                                </label>
                                <label class=label_class>
                                    "Best"
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
                    <div class="flex justify-center m-2 pb-2">
                        <input
                            type="submit"
                            class="btn btn-accent text-white rounded-md"
                            value="Start practice!"
                        />
                    </div>
                </form>
            </div>
        </div>
    }
}
