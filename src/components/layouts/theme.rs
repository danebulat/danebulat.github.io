use crate::components::ui::theme_toggle::ThemeToggle;
use leptos::prelude::*;

#[component]
pub fn ThemeSwitch() -> impl IntoView {
    view! {
        <div class="mt-1 pr-4">
            <ThemeToggle />
        </div>
    }
}
