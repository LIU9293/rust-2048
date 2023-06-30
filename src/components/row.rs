use dioxus::prelude::*;
use crate::components::cell::Cell;

#[derive(PartialEq, Props)]
pub struct RowProps {
    cells: [i32; 4],
}

pub fn Row (cx: Scope<RowProps>) -> Element {
    cx.render(rsx!(
        div {
            class: "flex flex-row gap-2",
            (0..4).map(|i| rsx!{ Cell { score: cx.props.cells[i] } })
        }
    ))
}
