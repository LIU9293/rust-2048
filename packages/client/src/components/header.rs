use dioxus::prelude::*;
use dioxus::html::MouseEvent;
use fermi::*;
use dioxus_html_macro::html;
use shared::translate::{TRANSLATION};

#[cfg(not(target_arch = "wasm32"))]
use dioxus_desktop::use_eval;
#[cfg(target_arch = "wasm32")]
use dioxus_web::use_eval;


#[derive(PartialEq, Props)]
pub struct ThemeButtonProps {
    theme: String,
}
pub fn ThemeButton (cx: Scope<ThemeButtonProps>) -> Element {
    let change_theme_to_light = move |_| {
        let eval = use_eval(&cx);
        let target_theme = match cx.props.theme.clone() == "light" {
            true => "lofi",
            false => "black",
        };

        let js_code = format!("
            document.documentElement.setAttribute('data-theme', '{}');
            localStorage.setItem('theme', '{}');
        ", target_theme, cx.props.theme.clone());

        let system_theme_js_code = r#"
            try {
                var preference_query = window.matchMedia('(prefers-color-scheme: dark)');
                function checkPreference(query) {
                    if (query.matches) {
                        document.documentElement.setAttribute('data-theme', 'black');
                        localStorage.setItem('theme', 'system');
                    } else {
                        document.documentElement.setAttribute('data-theme', 'lofi');
                        localStorage.setItem('theme', 'system');
                    }
                }
                checkPreference(preference_query);
                preference_query.addEventListener("change", checkPreference);
            } catch (_) { }
        "#;

        if cx.props.theme.clone() == "system" {
            eval(system_theme_js_code.to_string());
        } else {
            eval(js_code);
        }
    };

    cx.render(rsx!(
        li {
            onclick: change_theme_to_light,
            a {
                cx.props.theme.clone()
            }
        }
    ))
}


#[derive(Props)]
pub struct LanguageButtonProps<'a> {
    language: String,
    onclick: EventHandler<'a, MouseEvent>,
}
pub fn LanguageButton<'a> (cx: Scope<'a, LanguageButtonProps<'a>>) -> Element<'a> {
    cx.render(rsx!(
        li {
            onclick: move |evt| cx.props.onclick.call(evt),
            a {
                cx.props.language.clone()
            }   
        }
    ))
}


pub fn Header(cx: Scope) -> Element {
    let set_translator = use_set(cx, TRANSLATION);
    let original = use_read(cx, TRANSLATION);

    cx.render(html!(
        <div class="navbar bg-base-100">
            <div class="flex-1 font-bold px-4">
               <a href="/">
                    <div class="text-xl">"Rust2048"</div>
                </a>
            </div>
            <div class="flex-none">
                <ul class="menu menu-horizontal px-1 items-center gap-2">
                    <li><a>"Leaderboard"</a></li>
                    <li>
                        <details>
                            <summary>"Theme"</summary>
                            <ul class="p-2 bg-base-100">
                                <ThemeButton theme={"light".to_string()} />
                                <ThemeButton theme={"dark".to_string()} />
                                <ThemeButton theme={"system".to_string()} />
                            </ul>
                        </details>
                    </li>
                    <li>
                        <details>
                            <summary>"Language"</summary>
                            <ul class="p-2 bg-base-100">
                                <LanguageButton 
                                    language={"EN".to_string()}
                                    onclick={|_| {
                                        set_translator(original.change_to("en".to_string()))
                                    }}
                                />
                                <LanguageButton 
                                    language={"ZH".to_string()}
                                    onclick={|_| {
                                        set_translator(original.change_to("zh".to_string()))
                                    }}
                                />
                            </ul>
                        </details>
                    </li>
                </ul>
            </div>
        </div>
    ))
}
