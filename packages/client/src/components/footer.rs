use dioxus::prelude::*;
use dioxus_html_macro::html;

pub fn Footer(cx: Scope) -> Element {
    cx.render(html!(
        <footer class="footer footer-center p-10 bg-base-100 text-base-content rounded">
            <div class="grid grid-flow-col gap-4">
              <a target="_blank" href="https://github.com/LIU9293/rust-2048" class="link link-hover">"GitHub"</a>
              <a target="_blank" href="https://space.bilibili.com/588977169" class="link link-hover">"Bilibili"</a>
            </div>
            <div>
                <p>"Copyright © 2023 - KaiXiaoZao"</p>
            </div>
        </footer>
    ))
}