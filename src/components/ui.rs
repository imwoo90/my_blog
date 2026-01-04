use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Container(children: Element, class: Option<String>) -> Element {
    let base_class =
        "layout-content-container flex flex-col w-full max-w-5xl mx-auto px-4 sm:px-6 lg:px-8";
    let combined_class = if let Some(extra) = class {
        format!("{} {}", base_class, extra)
    } else {
        base_class.to_string()
    };

    rsx! {
        div { class: "{combined_class}", {children} }
    }
}

#[component]
pub fn Section(children: Element, class: Option<String>) -> Element {
    let combined_class = if let Some(extra) = class {
        format!("flex flex-col gap-6 {}", extra)
    } else {
        "flex flex-col gap-6".to_string()
    };

    rsx! {
        section { class: "{combined_class}", {children} }
    }
}

#[component]
pub fn Hero(
    title: String,
    subtitle: Option<String>,
    centered: Option<bool>,
    children: Element,
) -> Element {
    let is_centered = centered.unwrap_or(true);
    let align_class = if is_centered {
        "text-center items-center"
    } else {
        "text-left items-start"
    };

    rsx! {
        div { class: "w-full flex flex-col gap-6 {align_class} px-4 py-10 md:py-16",
            div { class: "flex flex-col gap-4",
                h1 { class: "text-white text-4xl md:text-6xl font-black leading-tight tracking-[-0.033em]",
                    "{title}"
                }
                if let Some(sub) = subtitle {
                    p { class: "text-[#D4D4D4] text-lg md:text-xl font-normal leading-normal max-w-3xl mx-auto",
                        "{sub}"
                    }
                }
            }
            {children}
        }
    }
}

#[component]
pub fn Card(
    title: String,
    description: String,
    image_url: String,
    tags: Vec<String>,
    link_to: Option<Route>,
    external_link: Option<String>,
    link_text: Option<String>,
) -> Element {
    let link_label = link_text.unwrap_or_else(|| "Read More".to_string());

    let content = rsx! {
        div { class: "w-full aspect-video bg-cover bg-center rounded-t-lg",
            style: "background-image: url('{image_url}')"
        }
        div { class: "p-6 flex flex-col flex-grow",
            h3 { class: "text-white text-xl font-bold leading-tight tracking-[-0.015em] group-hover:text-primary-light transition-colors",
                "{title}"
            }
            p { class: "text-[#D4D4D4] text-base font-normal leading-normal mt-2 mb-4 flex-grow",
                "{description}"
            }
            div { class: "flex flex-wrap items-center gap-2 mb-4",
                for tag in tags {
                    Badge { text: tag }
                }
            }
            if link_to.is_some() {
                div { class: "text-primary-light text-sm font-semibold hover:underline flex items-center gap-1",
                    "{link_label} "
                    span { class: "material-symbols-outlined text-base", "arrow_forward" }
                }
            } else if let Some(url) = external_link {
                a {
                    class: "text-primary-light text-sm font-semibold hover:underline flex items-center gap-1",
                    href: "{url}",
                    "{link_label} "
                    span { class: "material-symbols-outlined text-base", "arrow_forward" }
                }
            }
        }
    };

    if let Some(target) = link_to.clone() {
        rsx! {
            Link {
                to: target,
                class: "flex flex-col rounded-lg overflow-hidden bg-[#2a2a2a] border border-white/10 group transition-all duration-300 hover:bg-white/5",
                {content}
            }
        }
    } else {
        rsx! {
            div { class: "flex flex-col rounded-lg overflow-hidden bg-[#2a2a2a] border border-white/10 group transition-all duration-300 hover:bg-white/5",
                {content}
            }
        }
    }
}

#[component]
pub fn Badge(text: String) -> Element {
    rsx! {
        span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded",
            "{text}"
        }
    }
}

#[component]
pub fn PrimaryButton(
    text: String,
    to: Option<Route>,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let class = "flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-12 px-6 bg-primary-light text-[#1E1E1E] text-base font-bold leading-normal tracking-[0.015em] hover:opacity-90 transition-opacity";

    if let Some(route) = to.clone() {
        rsx! {
            Link { to: route, class: "{class}",
                span { class: "truncate", "{text}" }
            }
        }
    } else {
        rsx! {
            button {
                class: "{class}",
                onclick: move |e| {
                    if let Some(handler) = onclick {
                        handler.call(e);
                    }
                },
                span { class: "truncate", "{text}" }
            }
        }
    }
}
