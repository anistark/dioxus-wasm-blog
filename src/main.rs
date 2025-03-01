use dioxus::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        div { class: "text-center text-2xl font-bold",
            "Welcome to My Dioxus Blog"
        }
    }
}
