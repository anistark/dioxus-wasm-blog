use dioxus::prelude::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center min-h-screen w-full bg-gradient-to-br from-blue-900 to-gray-900",
            div {
                class: "text-center p-8 rounded-lg shadow-xl bg-gray-800 bg-opacity-50",
                h1 {
                    class: "text-5xl font-bold text-white drop-shadow-lg",
                    "Welcome to My Dioxus Blog"
                },
                p {
                    class: "text-lg text-gray-300 mt-4",
                    "A minimal blog powered by Dioxus and TailwindCSS!"
                },
                button {
                    class: "mt-6 px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-lg shadow-md transition",
                    "Read More"
                }
            }
        }
    }
}
