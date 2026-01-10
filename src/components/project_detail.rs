use crate::posts::markdown_to_html;
use crate::projects_data::get_project_by_id;
use crate::views::Footer;
use crate::Route;
use dioxus::prelude::*;

use crate::components::BlogHero;

#[component]
pub fn ProjectDetail(id: String) -> Element {
    let project = get_project_by_id(&id);

    match project {
        Some(project) => {
            let html_content = markdown_to_html(&project.content);
            rsx! {
                div { class: "layout-content-container flex flex-col w-full max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12 sm:py-16",
                    article { class: "w-full max-w-3xl flex flex-col gap-10",
                        BlogHero {
                            title: project.meta.title.clone(),
                            author: project.meta.author.clone(),
                            date: project.meta.date.clone(),
                            read_time: project.get_read_time(),
                        }

                        div {
                            class: "prose max-w-none dark:prose-invert",
                            dangerous_inner_html: "{html_content}"
                        }

                        if let Some(link) = &project.meta.link {
                            div { class: "mt-8",
                                a {
                                    href: "{link}",
                                    class: "inline-flex items-center gap-2 bg-primary-light text-white px-6 py-3 rounded-lg font-bold hover:bg-primary-dark transition-colors",
                                    target: "_blank",
                                    span { class: "material-symbols-outlined", "launch" }
                                    "{project.meta.link_text.clone().unwrap_or_else(|| \"Visit Project\".to_string())}"
                                }
                            }
                        }
                    }
                }
                Footer {}
            }
        }
        None => rsx! {
            div { class: "flex flex-col items-center justify-center min-h-[60vh]",
                h1 { class: "text-4xl font-bold", "Project Not Found" }
                Link {
                    to: Route::Projects {},
                    class: "mt-4 text-primary-light hover:underline",
                    "Back to Projects"
                }
            }
            Footer {}
        },
    }
}
