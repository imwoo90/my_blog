use crate::components::*;
use crate::projects_data::get_all_projects;
use crate::views::Footer;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    let projects = get_all_projects();

    rsx! {
        Container {
            main { class: "flex flex-col gap-12 mt-8 md:mt-16",
                Hero {
                    title: "The Workshop",
                    subtitle: "A collection of my work, showcasing the versatility of Rust across the full stack.",
                    centered: Some(false),
                    children: rsx! {},
                }
                Section { class: "px-4 mb-20",
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                        for project in projects {
                            Card {
                                title: project.title.clone(),
                                description: project.description.clone(),
                                image_url: project.image_url.clone(),
                                tags: project.tags.clone(),
                                link_text: project.link_text.clone().unwrap_or_else(|| "View Details".to_string()),
                                external_link: if project.route.is_none() && project.link.is_some() { project.link.clone().unwrap_or_else(|| "#".to_string()) } else { "".to_string() },
                                link_to: Some(Route::ProjectDetail {
                                    id: project.id.clone(),
                                }),
                            }
                        }
                    }
                }
            }
        }
        Footer {}
    }
}
