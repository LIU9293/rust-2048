use fermi::*;
use dioxus::prelude::*;
use dioxus_router::Link;
use dioxus_html_macro::html;
use shared::translate::TRANSLATION;

pub fn Homepage(cx: Scope) -> Element {
    let translator = use_read(cx, TRANSLATION);
    let title = translator.get("homepage.title".to_string());
    let subtitle = translator.get("homepage.subtitle".to_string());
    let start_game = translator.get("homepage.start_game".to_string());

    cx.render(html!(
        <div class="hero py-48 bg-base-200">
            <div class="hero-content text-center">
                <div class="max-w-md">
                <h1 class="text-5xl font-bold">{title}</h1>
                <p class="py-6">{subtitle}</p>
                <Link to="/game">
                    <button class="btn btn-primary">{start_game}</button>
                </Link>
                </div>
            </div>
        </div>
    ))
}