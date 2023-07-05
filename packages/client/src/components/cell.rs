use dioxus::prelude::*;

const COLOR_MAP: [(i32, &str, &str); 12] = [
    (0, "hsla(30,37%,89%,.35)", "#776e65"),
    (2, "#eee4da", "#776e65"),
    (4, "#ede0c8", "#776e65"),
    (8, "#f2b179", "#f9f6f2"),
    (16, "#f59563", "#f9f6f2"),
    (32, "#f67c5f", "#f9f6f2"),
    (64, "#f65e3b", "#f9f6f2"),
    (128, "#edcf72", "#f9f6f2"),
    (256, "#edcc61", "#f9f6f2"),
    (512, "#edc850", "#f9f6f2"),
    (1024, "#edc53f", "#f9f6f2"),
    (2048, "#edc22e", "#f9f6f2"),
];

fn get_colors_from_score (score: i32) -> (&'static str, &'static str) {
    for &(value, bg, text) in COLOR_MAP.iter() {
        if value == score {
            return (bg, text);
        }
    }
    ("#eee4da", "#776e65")
}


#[derive(PartialEq, Props)]
pub struct BlockProps {
    score: i32,
}

pub fn Cell(cx: Scope<BlockProps>) -> Element {
    let base_class = "text-xl bg-violet-300 flex justify-center items-center rounded-sm w-16 h-16";
    let (bg, text) = get_colors_from_score(cx.props.score);
    let cell_style = format!(
        "transition: all .15s ease;
        color: {};
        background-color: {};", text, bg);

    cx.render(rsx!(
        div {
            class: "{base_class}",
            style: "{cell_style}",
            match cx.props.score {
                0 => rsx!(p{}),
                _ => rsx!(p{format!("{}", cx.props.score)}),
            }
        }
    ))
}
