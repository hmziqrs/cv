// use crate::data::header::{get_header, SocialIcon};
use dioxus::prelude::*;

// src/data/header.rs
use chrono::{Datelike, Utc};

pub enum SocialIcon {
    Gmail,
    Telegram,
    X,
    LinkedIn,
}

pub struct SocialLink {
    pub href: String,
    pub icon: SocialIcon,
    pub label: String,
    pub target: Option<String>,
}

pub struct Header {
    pub links: Vec<SocialLink>,
    pub name: String,
    pub title: String,
    pub intro: Vec<String>,
}

pub fn get_header() -> Header {
    let current_year = Utc::now().year() as usize;
    let years_experience = current_year - 2017;

    Header {
        links: vec![
            SocialLink {
                href: "mailto:hmziqrs@gmail.com".to_string(),
                icon: SocialIcon::Gmail,
                label: "hmziqrs@gmail.com".to_string(),
                target: None,
            },
            SocialLink {
                href: "https://t.me/hmziqrs".to_string(),
                icon: SocialIcon::Telegram,
                label: "@hmziqrs".to_string(),
                target: None,
            },
            SocialLink {
                href: "https://x.com/hmziqrs".to_string(),
                icon: SocialIcon::X,
                label: "@hmziqrs".to_string(),
                target: None,
            },
            SocialLink {
                href: "https://www.linkedin.com/in/hmziqrs".to_string(),
                icon: SocialIcon::LinkedIn,
                label: "@hmziqrs".to_string(),
                target: Some("_linkedin".to_string()),
            },
        ],
        name: "Hamza Iqbal".to_string(),
        title: "Full Stack Engineer | Rust, Flutter, React Native".to_string(),
        intro: vec![
            format!("A self-taught engineer with {years_experience}+ years of experience, I've built different types of software solutions like travel booking platforms, social networks, online stores, and business applications. I mostly use Rust, Flutter, NextJS, NodeJS, and React Native."),
            "Outside of coding, I enjoy flying drones and playing story-rich video games like Detroit: Become Human and Cyber punk.".to_string(),
        ],
    }
}

pub fn AppHeader() -> Element {
    let header = get_header();

    rsx! {
        div { class: "flex bg-zinc-100 dark:bg-zinc-900",
            div { class: "container max-w-8xl mx-auto flex flex-col px-6 py-10 gap-6",
                div {
                    h1 { class: "text-3xl font-semibold text-zinc-800 dark:text-zinc-100",
                        "{header.name}"
                    }
                    div { class: "h-1" }
                    h3 { class: "font-semibold font-mono text-zinc-700 dark:text-zinc-300",
                        "{header.title}"
                    }
                    div { class: "h-4" }
                    div { class: "flex flex-col gap-y-4",
                        {header.intro.iter().map(|intro| {
                            rsx! {
                                p { class: "text-zinc-600 dark:text-zinc-400",
                                    "{intro}"
                                }
                            }
                        })}
                    }
                }
                div { class: "flex flex-row gap-4 flex-wrap",
                    {header.links.iter().map(|link| {
                        let target = link.target.as_deref().unwrap_or("_blank");
                        rsx! {
                            a {
                                key: "{link.href}",
                                href: "{link.href}",
                                target: "{target}",
                                class: "flex items-center gap-3 px-6 py-3 bg-zinc-200 dark:bg-zinc-800 hover:bg-zinc-300 dark:hover:bg-zinc-700 text-zinc-800 dark:text-zinc-200 rounded-md transition-colors duration-300",

                                // Render the appropriate icon based on the icon type
                                match link.icon {
                                    SocialIcon::Gmail => rsx! {
                                        svg {
                                            class: "w-[15px] h-[15px]",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            view_box: "0 0 24 24",
                                            fill: "currentColor",
                                            // Gmail icon path
                                            path { d: "M24 5.457v13.909c0 .904-.732 1.636-1.636 1.636h-3.819V11.73L12 16.64l-6.545-4.91v9.273H1.636A1.636 1.636 0 0 1 0 19.366V5.457c0-2.023 2.309-3.178 3.927-1.964L5.455 4.64 12 9.548l6.545-4.91 1.528-1.145C21.69 2.28 24 3.434 24 5.457z" }
                                        }
                                    },
                                    SocialIcon::Telegram => rsx! {
                                        svg {
                                            class: "w-[15px] h-[15px]",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            view_box: "0 0 24 24",
                                            fill: "currentColor",
                                            // Telegram icon path
                                            path { d: "M24 24h-24v-24h24v24zm-17.58-5.42c.338.102.636.032.881-.117.19-.116 1.596-1.587 1.606-1.597.423-.4 3.631-3.458 4.29-4.115.42-.42.753-.39 1.189.02l2.045 1.917.171.143c.232.173.548.16.784-.033l5.18-4.122c.344-.293.316-.818-.055-1.035l-3.26-1.912c-.391-.21-.755-.06-1.053.225-2.02 1.936-4.67 4.264-6.961 6.573l-3.265 3.102c-.237.225-.392.538-.37.849.24.345.242.61.46.814.215.203.508.287.837.288.096 0 .194-.013.291-.04z" }
                                        }
                                    },
                                    SocialIcon::X => rsx! {
                                        svg {
                                            class: "w-[15px] h-[15px]",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            view_box: "0 0 24 24",
                                            fill: "currentColor",
                                            // X (Twitter) icon path
                                            path { d: "M18.901 1.153h3.68l-8.04 9.19L24 22.846h-7.406l-5.8-7.584-6.638 7.584H.474l8.6-9.83L0 1.154h7.594l5.243 6.932ZM17.61 20.644h2.039L6.486 3.24H4.298Z" }
                                        }
                                    },
                                    SocialIcon::LinkedIn => rsx! {
                                        svg {
                                            class: "w-[15px] h-[15px]",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            view_box: "0 0 24 24",
                                            fill: "currentColor",
                                            // LinkedIn icon path
                                            path { d: "M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z" }
                                        }
                                    },
                                },

                                p { class: "font-medium text-sm",
                                    "{link.label}"
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}
