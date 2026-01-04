use crate::components::*;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        div { class: "p-8 text-center",
            h1 { class: "text-2xl font-bold", "404 - Page Not Found" }
            p { "Path attempted: {segments.join(\"/\")}" }
            a { href: "/my_blog/", class: "text-blue-500 underline", "Go Home" }
        }
    }
}
