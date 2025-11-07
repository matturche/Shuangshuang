use leptos::prelude::*;

// A component to render the help dropdown window on the form
#[component]
pub fn SubtitleWithHelper<F, IV>(
    subtitle: String,
    helper_title: String,
    helper_desc: F,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    let subtitle_font_class = "rounded-sm bg-neutral underline px-1";
    view! {
        <legend>
            <a class=subtitle_font_class>{subtitle}</a>
            <a>
                <div class="dropdown dropdown-center">
                    <div
                        tabindex="0"
                        role="button"
                        class="btn btn-circle btn-ghost btn-xs text-secondary"
                    >
                        "?"
                    </div>
                    <div
                        tabindex="0"
                        class="card card-sm dropdown-content bg-base-100 rounded-box z-1 w-64 h-40 shadow-sm overflow-auto"
                    >
                        <div tabindex="0" class="card-body">
                            <h2 class="card-title text-sm">{helper_title}</h2>
                            <p>{helper_desc()}</p>
                        </div>
                    </div>
                </div>

            </a>
        </legend>
    }
}
