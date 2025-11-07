use std::str::FromStr;

use crate::i18n::*;
use crate::LOCALES;

use leptos::html;
use leptos::prelude::*;
use leptos::tachys::html::event::Event;

#[component]
pub fn LanguageController() -> impl IntoView {
    let i18n = use_i18n();
    let (preferred_language, _set_preferred_language) =
        signal(i18n.get_locale_untracked().to_string());
    let selected_language: NodeRef<html::Select> = NodeRef::new();
    // NOTE: This handles language selection
    let on_lang_select = {
        move || {
            let language = selected_language
                .get()
                .expect("<select> should be mounted for preferred language.")
                .value()
                .to_lowercase();
            let locale =
                Locale::from_str(&language).expect("Failed to build locale from str value.");
            i18n.set_locale(locale);
        }
    };

    view! {
        <select
            class="select select-secondary w-fit mx-2"
            node_ref=selected_language
            id="pref-lang"
            on:change=move |_ev: Event| {
                on_lang_select();
            }
        >
            <option disabled>"Lang"</option>
            {move || {
                LOCALES
                    .iter()
                    .map(|locale| {
                        if &preferred_language() == *locale {
                            view! { <option selected>{locale.to_uppercase()}</option> }.into_any()
                        } else {
                            view! { <option>{locale.to_uppercase()}</option> }.into_any()
                        }
                    })
                    .collect_view()
            }}
        </select>
    }
    .into_any()
}
