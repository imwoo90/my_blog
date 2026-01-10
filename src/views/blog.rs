use crate::components::*;
use crate::posts::{get_all_categories, get_all_posts, get_post_by_id, markdown_to_html};
use crate::views::Footer;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn BlogList() -> Element {
    let posts = get_all_posts();

    rsx! {
        Container {
            main { class: "flex flex-col gap-12 mt-8 md:mt-16",
                Hero {
                    title: "From the Horizon",
                    subtitle: "Exploring the frontiers of Rust, from bare-metal to the web. Find articles, tutorials, and deep dives here.",
                }

                // Filter / Search Section
                section { class: "flex flex-col md:flex-row gap-4 px-4 items-center",
                    BlogSearch { placeholder: "Search articles..." }
                    BlogCategories {
                        categories: get_all_categories(),
                        active: "All".to_string(),
                    }
                }

                Section { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                    for post in posts {
                        Card {
                            title: post.title.clone(),
                            description: post.description.clone(),
                            image_url: post.image_url.clone(),
                            tags: post.tags.clone(),
                            link_to: Route::BlogPost {
                                id: post.id.clone(),
                            },
                        }
                    }
                }
            }
        }
        Footer {}
    }
}

#[component]
pub fn BlogPost(id: String) -> Element {
    let post = get_post_by_id(&id);

    use_effect(move || {
        document::eval("if (window.hljs) window.hljs.highlightAll();");
    });

    match post {
        Some(post) => {
            let html_content = markdown_to_html(&post.content);
            rsx! {
                div { class: "layout-content-container flex flex-col w-full max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12 sm:py-16",
                    article { class: "w-full max-w-3xl flex flex-col gap-10",
                        BlogHero {
                            title: post.meta.title.clone(),
                            author: post.meta.author.clone(),
                            date: post.meta.date.clone(),
                            read_time: post.get_read_time(),
                        }

                        div {
                            class: "prose max-w-none dark:prose-invert",
                            dangerous_inner_html: "{html_content}"
                        }
                    }

                    // Share Section
                    div { class: "mt-8 border-t border-text-dark/5 dark:border-white/5 pt-8",
                        div { class: "flex flex-col sm:flex-row justify-between items-center gap-4 bg-text-dark/5 dark:bg-white/5 p-6 rounded-xl",
                            h3 { class: "text-lg font-bold text-text-dark dark:text-white",
                                "Share this article"
                            }
                            div { class: "flex items-center gap-3",
                                button { class: "text-text-dark/40 dark:text-gray-400 hover:text-text-dark dark:hover:text-white hover:bg-text-dark/10 dark:hover:bg-[#1DA1F2]/20 p-2.5 rounded-lg transition-all",
                                    span { class: "material-symbols-outlined", "share" }
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
                h1 { class: "text-4xl font-bold", "Post Not Found" }
                Link {
                    to: Route::BlogList {},
                    class: "mt-4 text-primary-light hover:underline",
                    "Back to Blog"
                }
            }
            Footer {}
        },
    }
}
