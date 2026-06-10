use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

use crate::components::hooks::use_theme_mode::ThemeMode;
use crate::components::{Certificates, NavBar, Skills, Summary, Work};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Initialize the theme context once at app root.
    let _ = ThemeMode::init();

    view! {
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home />
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="flex flex-col min-h-screen">
            <section class="sticky top-0 z-50 bg-background">
                <NavBar />
            </section>
            <main class="flex-1">
                <div class="flex flex-col gap-4 px-2 pb-4">
                    <Summary />
                    <WorkAndCertificates />
                    <Skills />
                </div>
            </main>
            <footer class="mt-auto border-t bg-muted/20">
                <div class="max-w-3xl mx-auto px-4 py-4">
                    <p class="text-center text-[13px] text-muted-foreground/80">
                        "Copyright © 2026 Dane Bulat"
                    </p>
                </div>
            </footer>
        </div>
    }
}

#[component]
fn WorkAndCertificates() -> impl IntoView {
    view! {
        <div class="w-full sm:max-w-3xl sm:mx-auto">
            <div class="grid md:grid-cols-2 lg:grid-cols-2 gap-2">
                <Work />
                <Certificates />
            </div>
        </div>
    }
}
