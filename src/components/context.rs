use leptos::prelude::*;

use crate::i18n::*;

// A component to display Shuangshuang context info
#[component]
pub fn Context() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        {move || {
            view! {
                <article class="prose mx-2">
                    <h3>{t!(i18n, context.context_subtitle)}</h3>
                    <p>
                        {t!(i18n, context.app_context_1)}<br />{t!(i18n, context.app_context_2)}
                        <br /> {t!(i18n, context.app_context_3)}
                    </p>
                    <h3>{t!(i18n, context.starting_out_subtitle)}</h3>
                    <p>{t!(i18n, context.starting_out)}</p>
                    <h3>{t!(i18n, context.tone_rules_subtitle)}</h3>
                    <p>{t!(i18n, context.sandhi)}</p>
                </article>
            }
        }}
    }
}
