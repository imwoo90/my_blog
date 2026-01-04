use crate::components::*;
use crate::views::Footer;
use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        Container {
            Hero {
                title: "Get In Touch",
                subtitle: "Have a project in mind, a question about an article, or just want to connect? I'm always open to discussing new opportunities and collaborating on exciting ideas.",
                children: rsx! {}
            }

            Section { class: "px-4 mb-20",
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-8 md:gap-12",
                    div { class: "md:col-span-2 bg-white dark:bg-[#2a2a2a] p-8 rounded-lg border border-text-dark/5 dark:border-white/10 transition-colors shadow-sm",
                        form {
                            class: "flex flex-col gap-6",
                            onsubmit: |e| e.prevent_default(),
                            h2 { class: "text-text-dark dark:text-white text-2xl font-bold leading-tight tracking-[-0.015em] transition-colors",
                                "Send a Message"
                            }
                            div { class: "flex flex-col sm:flex-row gap-6",
                                div { class: "flex flex-col gap-2 w-full",
                                    label {
                                        class: "text-sm font-medium text-text-dark/60 dark:text-[#D4D4D4] transition-colors",
                                        r#for: "name",
                                        "Your Name"
                                    }
                                    input {
                                        class: "w-full bg-white dark:bg-background-dark border border-text-dark/10 dark:border-white/20 rounded-md h-11 px-4 text-base text-text-dark dark:text-white placeholder:text-text-dark/30 dark:placeholder:text-gray-500 focus:ring-primary-light focus:border-primary-light transition-all",
                                        id: "name",
                                        placeholder: "John Doe",
                                        r#type: "text",
                                    }
                                }
                                div { class: "flex flex-col gap-2 w-full",
                                    label {
                                        class: "text-sm font-medium text-text-dark/60 dark:text-[#D4D4D4] transition-colors",
                                        r#for: "email",
                                        "Your Email"
                                    }
                                    input {
                                        class: "w-full bg-white dark:bg-background-dark border border-text-dark/10 dark:border-white/20 rounded-md h-11 px-4 text-base text-text-dark dark:text-white placeholder:text-text-dark/30 dark:placeholder:text-gray-500 focus:ring-primary-light focus:border-primary-light transition-all",
                                        id: "email",
                                        placeholder: "john.doe@email.com",
                                        r#type: "email",
                                    }
                                }
                            }
                            div { class: "flex flex-col gap-2",
                                label {
                                    class: "text-sm font-medium text-text-dark/60 dark:text-[#D4D4D4] transition-colors",
                                    r#for: "message",
                                    "Message"
                                }
                                textarea {
                                    class: "w-full bg-white dark:bg-background-dark border border-text-dark/10 dark:border-white/20 rounded-md p-4 text-base text-text-dark dark:text-white placeholder:text-text-dark/30 dark:placeholder:text-gray-500 focus:ring-primary-light focus:border-primary-light transition-all",
                                    id: "message",
                                    placeholder: "I'd like to discuss...",
                                    rows: "6",
                                }
                            }
                            div { class: "flex justify-start",
                                PrimaryButton {
                                    text: "Submit Message",
                                    onclick: move |_| {
                                        // TODO: Implement form submission
                                    }
                                }
                            }
                        }
                    }
                    div { class: "flex flex-col gap-6",
                        h2 { class: "text-text-dark dark:text-white text-2xl font-bold leading-tight tracking-[-0.015em] transition-colors",
                            "Contact Information"
                        }
                        div { class: "flex flex-col gap-4",
                            ContactInfoItem {
                                icon: "mail",
                                label: "Email",
                                value: "contact@rustshorizon.com",
                                href: "mailto:contact@rustshorizon.com"
                            }
                            ContactInfoItem {
                                icon: "link",
                                label: "GitHub",
                                value: "@rust-developer",
                                href: "https://github.com"
                            }
                            ContactInfoItem {
                                icon: "group",
                                label: "LinkedIn",
                                value: "Jane Doe",
                                href: "https://linkedin.com"
                            }
                        }
                    }
                }
            }
        }
        Footer {}
    }
}

#[component]
fn ContactInfoItem(icon: String, label: String, value: String, href: String) -> Element {
    rsx! {
        div { class: "flex items-center gap-4",
            div { class: "flex items-center justify-center size-10 bg-text-dark/5 dark:bg-white/10 rounded-full text-primary-light transition-colors",
                span { class: "material-symbols-outlined", "{icon}" }
            }
            div { class: "flex flex-col",
                p { class: "text-sm text-text-dark/40 dark:text-gray-400 transition-colors", "{label}" }
                a {
                    class: "text-base font-medium text-text-dark dark:text-white hover:text-primary-light transition-colors",
                    href: "{href}",
                    "{value}"
                }
            }
        }
    }
}
