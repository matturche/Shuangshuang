use leptos::prelude::*;

use crate::i18n::*;

// A component to display Shuangshuang context info
#[component]
pub fn Context() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        {move || {
            view! {
                <article class="prose">
                    <h3>"What is Shuangshuang?"</h3>
                    <p>
                        {t!(i18n, context.app_context_1)}<br />{t!(i18n, context.app_context_2)}
                        <br /> {t!(i18n, context.app_context_3)}
                    </p>
                    <h3>"Starting out"</h3>
                    <p>{t!(i18n, context.starting_out)}</p>
                    <h3>"Tone rules"</h3>
                    <p>{t!(i18n, context.sandhi)}</p>
                </article>
            }
        }}
    }
}
