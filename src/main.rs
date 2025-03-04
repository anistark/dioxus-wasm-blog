use dioxus::prelude::*;

use components::Navbar;
use views::{Blog, Home};

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS },
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
