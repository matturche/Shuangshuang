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
    view! {
        <form on:submit=on_submit>
            <div>
                <label for="nb_elements">"Number of test questions"</label>
                <br />
                <input
                    type="range"
                    min=nb_elements_min
                    max=nb_elements_max
                    step=nb_elements_step
                    value=nb_elements
                    node_ref=nb_elements_element
                    list="values"
                />

                <datalist id="values">
                    {
                        let mut datalist_options_views: Vec<AnyView> = vec![];
                        for element_step in (nb_elements_min..nb_elements_max + 1)
                            .step_by(nb_elements_step)
                        {
                            datalist_options_views
                                .push(
                                    view! {
                                        <option value=element_step label=element_step></option>
                                    }
                                        .into_any(),
                                )
                        }
                        datalist_options_views.collect_view()
                    }
                </datalist>

            </div>
            <div>
                <fieldset>
                    <legend>"Select exercise type"</legend>
                    <label>
                        "Tone Only"
                        <input
                            type="radio"
                            value=ExerciseType::ToneOnly.to_string()
                            bind:group=exercise_type
                        />
                    </label>
                    <label>
                        "Pinyin"
                        <input
                            type="radio"
                            value=ExerciseType::Pinyin.to_string()
                            bind:group=exercise_type
                        />
                    </label>
                </fieldset>
            </div>
            <div>
                <fieldset>
                    <legend>"Select input style"</legend>

                    {move || {
                        if let ExerciseType::ToneOnly = ExerciseType::from_str(&exercise_type())
                            .unwrap()
                        {
                            view! {
                                <label>
                                    "Keyboard"
                                    <input
                                        type="radio"
                                        value=InputStyle::Keyboard.to_string()
                                        bind:group=input_style
                                    />
                                </label>
                                <label>
                                    "Buttons"
                                    <input
                                        type="radio"
                                        value=InputStyle::Touch.to_string()
                                        bind:group=input_style
                                    />
                                </label>
                            }
                                .into_any()
                        } else {
                            input_style.set(InputStyle::Keyboard.to_string());
                            view! {
                                <label>
                                    "Keyboard"
                                    <input
                                        disabled
                                        type="radio"
                                        value=InputStyle::Keyboard.to_string()
                                        bind:group=input_style
                                    />
                                </label>
                                <label>
                                    "Buttons"
                                    <input
                                        disabled
                                        type="radio"
                                        value=InputStyle::Touch.to_string()
                                        bind:group=input_style
                                    />
                                </label>
                            }
                                .into_any()
                        }
                    }}
                </fieldset>
            </div>
            <div>
                <fieldset>
                    <legend>"Select audio quality"</legend>
                    <label>
                        "Low"
                        <input
                            type="radio"
                            value=AudioQuality::Q18k.to_string()
                            bind:group=audio_quality
                        />
                    </label>
                    <label>
                        "Medium"
                        <input
                            type="radio"
                            value=AudioQuality::Q24k.to_string()
                            bind:group=audio_quality
                        />
                    </label>
                    <label>
                        "High"
                        <input
                            type="radio"
                            value=AudioQuality::Q64k.to_string()
                            bind:group=audio_quality
                        />
                    </label>
                    <label>
                        "Best"
                        <input
                            type="radio"
                            value=AudioQuality::Q96k.to_string()
                            bind:group=audio_quality
                        />
                    </label>
                </fieldset>
            </div>
            <input type="submit" value="Start practice!" />
        </form>
    }
}
