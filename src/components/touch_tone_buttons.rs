use leptos::{html, prelude::*};

use crate::exercise::Tone;
use crate::i18n::*;
use crate::utils::format_toned_syllable_url;

// A component to display all tones and play the syllable audio when clicked
#[component]
pub fn TouchToneButtons(
    pronounced_pinyin: ReadSignal<String>,
    first_tone_value: RwSignal<String>,
    second_tone_value: RwSignal<String>,
    set_audio_playing: WriteSignal<bool>,
) -> impl IntoView {
    let i18n = use_i18n();

    let (first_syllable, set_first_syllable) = signal("".to_string());
    let (second_syllable, set_second_syllable) = signal("".to_string());

    let audio_element_first_s_first_t: NodeRef<html::Audio> = NodeRef::new();
    let audio_element_first_s_second_t: NodeRef<html::Audio> = NodeRef::new();
    let audio_element_first_s_third_t: NodeRef<html::Audio> = NodeRef::new();
    let audio_element_first_s_fourth_t: NodeRef<html::Audio> = NodeRef::new();
    let audio_element_second_s_first_t: NodeRef<html::Audio> = NodeRef::new();
    let audio_element_second_s_second_t: NodeRef<html::Audio> = NodeRef::new();
    let audio_element_second_s_third_t: NodeRef<html::Audio> = NodeRef::new();
    let audio_element_second_s_fourth_t: NodeRef<html::Audio> = NodeRef::new();

    let play_first_s_first_t_audio = move |_| {
        let audio = audio_element_first_s_first_t
            .get()
            .expect("<audio> first_s_first_t element should be mounted");
        let _promise = audio
            .play()
            .expect("Failed to play audio first_s_first_t element.");
        set_audio_playing(true);
    };
    let play_first_s_second_t_audio = move |_| {
        let audio = audio_element_first_s_second_t
            .get()
            .expect("<audio> first_s_second_t element should be mounted");
        let _promise = audio
            .play()
            .expect("Failed to play audio first_s_second_t element.");
        set_audio_playing(true);
    };
    let play_first_s_third_t_audio = move |_| {
        let audio = audio_element_first_s_third_t
            .get()
            .expect("<audio> first_s_third_t element should be mounted");
        let _promise = audio
            .play()
            .expect("Failed to play audio first_s_third_t element.");
        set_audio_playing(true);
    };
    let play_first_s_fourth_t_audio = move |_| {
        let audio = audio_element_first_s_fourth_t
            .get()
            .expect("<audio> first_s_fourth_t element should be mounted");
        let _promise = audio
            .play()
            .expect("Failed to play audio first_s_fourth_t element.");
        set_audio_playing(true);
    };
    let play_second_s_first_t_audio = move |_| {
        let audio = audio_element_second_s_first_t
            .get()
            .expect("<audio> second_s_firs_t element should be mounted");
        let _promise = audio
            .play()
            .expect("Failed to play audio second_s_first_t element.");
        set_audio_playing(true);
    };
    let play_second_s_second_t_audio = move |_| {
        let audio = audio_element_second_s_second_t
            .get()
            .expect("<audio> second_s_second_t element should be mounted");
        let _promise = audio
            .play()
            .expect("Failed to play audio second_s_second_t element.");
        set_audio_playing(true);
    };
    let play_second_s_third_t_audio = move |_| {
        let audio = audio_element_second_s_third_t
            .get()
            .expect("<audio> second_s_third_t element should be mounted");
        let _promise = audio
            .play()
            .expect("Failed to play audio second_s_third_t element.");
        set_audio_playing(true);
    };
    let play_second_s_fourth_t_audio = move |_| {
        let audio = audio_element_second_s_fourth_t
            .get()
            .expect("<audio> second_s_fourth_t element should be mounted");
        let _promise = audio
            .play()
            .expect("Failed to play audio second_s_fourth_t element.");
        set_audio_playing(true);
    };

    view! {
        {move || {
            let radio_class = "radio radio-sm radio-primary";
            let radio_space_class = "px-1";
            let spaced_no_tone_pinyin = pronounced_pinyin().replace(char::is_numeric, " ");
            let syllables: Vec<&str> = spaced_no_tone_pinyin.split(' ').collect();
            set_first_syllable(syllables[0].to_string());
            set_second_syllable(syllables[1].to_string());
            let first_tone = Tone::Tone1.to_string();
            let second_tone = Tone::Tone2.to_string();
            let third_tone = Tone::Tone3.to_string();
            let fourth_tone = Tone::Tone4.to_string();
            view! {
                <div>
                    <fieldset>
                        <legend class="fieldset-legend">
                            {t!(i18n, exercise.select_first_tone_value)}
                        </legend>
                        <label class=radio_space_class>
                            <audio
                                node_ref=audio_element_first_s_first_t
                                on:ended=move |_| { set_audio_playing(false) }
                            >
                                <source
                                    type="audio/mpeg"
                                    src=format_toned_syllable_url(&first_syllable(), &first_tone)
                                />
                            </audio>
                            {first_tone.clone()}
                            <input
                                type="radio"
                                class=radio_class
                                value=first_tone.clone()
                                bind:group=first_tone_value
                                on:click=play_first_s_first_t_audio
                                required
                            />
                        </label>
                        <label class=radio_space_class>
                            <audio
                                node_ref=audio_element_first_s_second_t
                                on:ended=move |_| { set_audio_playing(false) }
                            >
                                <source
                                    type="audio/mpeg"
                                    src=format_toned_syllable_url(&first_syllable(), &second_tone)
                                />
                            </audio>
                            {second_tone.clone()}
                            <input
                                type="radio"
                                class=radio_class
                                value=second_tone.clone()
                                bind:group=first_tone_value
                                on:click=play_first_s_second_t_audio
                                required
                            />
                        </label>
                        <label class=radio_space_class>
                            <audio
                                node_ref=audio_element_first_s_third_t
                                on:ended=move |_| { set_audio_playing(false) }
                            >
                                <source
                                    type="audio/mpeg"
                                    src=format_toned_syllable_url(&first_syllable(), &third_tone)
                                />
                            </audio>
                            {third_tone.clone()}
                            <input
                                type="radio"
                                class=radio_class
                                value=third_tone.clone()
                                bind:group=first_tone_value
                                on:click=play_first_s_third_t_audio
                                required
                            />
                        </label>
                        <label class=radio_space_class>
                            <audio
                                node_ref=audio_element_first_s_fourth_t
                                on:ended=move |_| { set_audio_playing(false) }
                            >
                                <source
                                    type="audio/mpeg"
                                    src=format_toned_syllable_url(&first_syllable(), &fourth_tone)
                                />
                            </audio>
                            {fourth_tone.clone()}
                            <input
                                type="radio"
                                class=radio_class
                                value=fourth_tone.clone()
                                bind:group=first_tone_value
                                on:click=play_first_s_fourth_t_audio
                                required
                            />
                        </label>

                    </fieldset>
                    <fieldset>
                        <legend class="fieldset-legend">
                            {t!(i18n, exercise.select_second_tone_value)}
                        </legend>
                        <label class=radio_space_class>
                            <audio
                                node_ref=audio_element_second_s_first_t
                                on:ended=move |_| { set_audio_playing(false) }
                            >
                                <source
                                    type="audio/mpeg"
                                    src=format_toned_syllable_url(&second_syllable(), &first_tone)
                                />
                            </audio>
                            {first_tone.clone()}
                            <input
                                type="radio"
                                class=radio_class
                                value=first_tone
                                bind:group=second_tone_value
                                on:click=play_second_s_first_t_audio
                                required
                            />
                        </label>
                        <label class=radio_space_class>
                            <audio
                                node_ref=audio_element_second_s_second_t
                                on:ended=move |_| { set_audio_playing(false) }
                            >
                                <source
                                    type="audio/mpeg"
                                    src=format_toned_syllable_url(&second_syllable(), &second_tone)
                                />
                            </audio>
                            {second_tone.clone()}
                            <input
                                type="radio"
                                class=radio_class
                                value=second_tone
                                bind:group=second_tone_value
                                on:click=play_second_s_second_t_audio
                                required
                            />
                        </label>
                        <label class=radio_space_class>
                            <audio
                                node_ref=audio_element_second_s_third_t
                                on:ended=move |_| { set_audio_playing(false) }
                            >
                                <source
                                    type="audio/mpeg"
                                    src=format_toned_syllable_url(&second_syllable(), &third_tone)
                                />
                            </audio>
                            {third_tone.clone()}
                            <input
                                type="radio"
                                class=radio_class
                                value=third_tone
                                bind:group=second_tone_value
                                on:click=play_second_s_third_t_audio
                                required
                            />
                        </label>
                        <label class=radio_space_class>
                            <audio
                                node_ref=audio_element_second_s_fourth_t
                                on:ended=move |_| { set_audio_playing(false) }
                            >
                                <source
                                    type="audio/mpeg"
                                    src=format_toned_syllable_url(&second_syllable(), &fourth_tone)
                                />
                            </audio>
                            {fourth_tone.clone()}
                            <input
                                type="radio"
                                class=radio_class
                                value=fourth_tone
                                bind:group=second_tone_value
                                on:click=play_second_s_fourth_t_audio
                                required
                            />
                        </label>
                        <label class=radio_space_class>
                            {Tone::NeutralTone.to_string()}
                            <input
                                type="radio"
                                class=radio_class
                                value=Tone::NeutralTone.to_string()
                                bind:group=second_tone_value
                                required
                            />
                        </label>
                    </fieldset>
                </div>
            }
        }}
    }
}
