use crate::api::fetch_hanzi_pairs;
use crate::components::context::Context;
use crate::components::language_controller::LanguageController;
use crate::components::test_form::TestForm;
use crate::components::test_session::TestSession;
use crate::components::theme_controller::ThemeController;
use crate::exercise::HanziPair;
use crate::i18n::*;
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let i18n = use_i18n();
    let (exercise_params, set_exercise_params) = signal(None);
    let (exercise_finished, set_exercise_finished) = signal(false);
    let fetched_hanzi_pairs = LocalResource::new(async move || fetch_hanzi_pairs().await);
    let (hanzi_pairs, set_hanzi_pairs) = signal::<Vec<HanziPair>>(vec![]);

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <Suspense fallback=move || {
                view! {
                    <div class="h-screen flex justify-center items-center">
                        <span class="loading loading-spinner text-primary"></span>
                    </div>
                }
            }>
                <div class="bg-base-200 h-screen">
                    <div class="flex justify-end p-2">
                        <ThemeController />
                        <LanguageController />
                    </div>

                    {move || {
                        Suspend::new(async move {
                            set_hanzi_pairs(fetched_hanzi_pairs.await);
                        })
                    }}
                    {move || {
                        if exercise_finished() {
                            set_exercise_params(None);
                            set_exercise_finished(false);
                        }
                        if exercise_params.read().is_some() {
                            view! {
                                <TestSession hanzi_pairs exercise_params set_exercise_finished />
                            }
                                .into_any()
                        } else {
                            let tab_class = "tab-content pt-2 pb-8";
                            view! {
                                <div class="flex flex-col justify-center py-4 text-center">
                                    <h1 class="text-2xl">{t!(i18n, intro.main_title)}</h1>
                                    <p class="text-lg">{t!(i18n, intro.sub_title)}</p>
                                </div>

                                <div class="flex justify-center bg-base-200">
                                    <div class="tabs tabs-border justify-center">
                                        <input
                                            type="radio"
                                            name="my_tabs_2"
                                            class="tab"
                                            aria-label=t_string!(i18n, form.exercise_tab)
                                            checked="checked"
                                        />
                                        <div class=tab_class>
                                            <TestForm set_exercise_params />
                                        </div>

                                        <input
                                            type="radio"
                                            name="my_tabs_2"
                                            class="tab"
                                            aria-label=t_string!(i18n, context.context_tab)
                                        />
                                        <div class=tab_class>
                                            <Context />
                                        </div>

                                        <input
                                            type="radio"
                                            name="my_tabs_2"
                                            class="tab"
                                            aria-label=t_string!(i18n, about.about_tab)
                                        />
                                        <div class=tab_class>
                                            // <div class="email">
                                            // should<span>.</span><span>gustave</span><span>put</span>
                                            // <span>@</span> <span>email</span><span>.</span>
                                            // <span>org</span>
                                            // </div>
                                            <div class="flex flex-row justify-center">
                                                <p>
                                                    {t!(i18n, about.free)}
                                                    <li>
                                                        {t!(i18n, about.source_code)}
                                                        <a
                                                            class="link"
                                                            href="https://github.com/matturche/Shuangshuang/"
                                                        >
                                                            "GitHub"
                                                        </a>
                                                    </li>
                                                    <li>
                                                        {t!(i18n, about.audio_origins)}
                                                        <a class="link" href="https://github.com/hugolpz/audio-cmn">
                                                            "GitHub"
                                                        </a>
                                                    </li>
                                                    <li>
                                                        {t!(i18n, about.pinyin_entries)}
                                                        <a class="link" href="https://cc-cedict.org/wiki/">
                                                            "CC-CEDICT"
                                                        </a>
                                                    </li>
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }
                                .into_any()
                        }
                    }}
                </div>
            </Suspense>
        </ErrorBoundary>
    }
}
