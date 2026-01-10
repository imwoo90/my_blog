use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct Comment {
    pub author: String,
    pub date: String,
    pub avatar_url: String,
    pub text: String,
}

#[component]
pub fn Comments(comments: Vec<Comment>) -> Element {
    rsx! {
        div { class: "mt-12",
            h2 { class: "text-2xl font-bold text-white mb-8", "Comments ({comments.len()})" }
            div { class: "space-y-6",
                for comment in comments {
                    CommentEntry {
                        author: comment.author.clone(),
                        date: comment.date.clone(),
                        avatar_url: comment.avatar_url.clone(),
                        text: comment.text.clone(),
                    }
                }
            }
            div { class: "mt-12",
                h3 { class: "text-lg font-semibold text-white mb-4", "Leave a comment" }
                form { class: "space-y-4",
                    div {
                        textarea {
                            class: "w-full bg-[#111111] border border-white/10 rounded-xl p-4 text-white placeholder:text-gray-600 focus:ring-2 focus:ring-primary focus:border-transparent transition-all outline-none resize-y min-h-[140px]",
                            placeholder: "Write your comment...",
                            rows: 4,
                        }
                    }
                    div { class: "flex justify-end",
                        button {
                            class: "bg-primary text-background-dark font-bold py-2.5 px-6 rounded-lg hover:bg-opacity-90 transition-colors shadow-lg",
                            "type": "submit",
                            "Post Comment"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CommentEntry(author: String, date: String, avatar_url: String, text: String) -> Element {
    rsx! {
        div { class: "flex gap-4",
            img {
                alt: "{author}",
                class: "size-10 rounded-full object-cover border border-white/10",
                src: "{avatar_url}",
            }
            div { class: "flex-1 bg-[#252525] rounded-xl p-5 border border-white/5",
                div { class: "flex justify-between items-start mb-2",
                    p { class: "font-semibold text-white text-sm", "{author}" }
                    p { class: "text-xs text-gray-500", "{date}" }
                }
                p { class: "text-gray-300 text-sm leading-relaxed", "{text}" }
            }
        }
    }
}
