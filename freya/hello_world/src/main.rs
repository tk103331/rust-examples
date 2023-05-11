#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    render!(
        container {
            height: "100%",
            width: "100%",
            background: "rgb(135, 135, 135)",
            color: "white",
            padding: "12",
            Button {
                onclick: move |_| count += 1,
                label { "Click to decrease  " }
            },
            label { "Count: {count}" }
        }
    )
}