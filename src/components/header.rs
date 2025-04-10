use dioxus::prelude::*;

// src/data/header.rs
use chrono::{Datelike, Utc};
use hmziq_dioxus_free_icons::icons::ld_icons::LdGlobe;
use hmziq_dioxus_free_icons::icons::si_icons::{SiGmail, SiTelegram, SiX};
use hmziq_dioxus_free_icons::{Icon, IconShape};

pub trait IconWithExtra: IconShape + Clone + PartialEq {}
impl<T> IconWithExtra for T where T: IconShape + Clone + PartialEq {}

pub enum SocialLinkIcon {
    Gmail,
    Telegram,
    X,
    Linkedin,
}

pub struct SocialLink {
    pub href: String,
    pub icon: Element,
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
                icon: rsx!{ Icon { icon: SiGmail } },
                // icon: SocialLinkIcon::Gmail,
                label: "hmziqrs@gmail.com".to_string(),
                target: None,
            },
            SocialLink {
                href: "https://t.me/hmziqrs".to_string(),
                icon: rsx!{ Icon { icon: SiTelegram } },
                // icon: SocialLinkIcon::Telegram,
                label: "@hmziqrs".to_string(),
                target: None,
            },
            SocialLink {
                href: "https://x.com/hmziqrs".to_string(),
                icon: rsx!{ Icon { icon: SiX } },
                // icon: SocialLinkIcon::X,
                label: "@hmziqrs".to_string(),
                target: None,
            },
            SocialLink {
                href: "https://www.linkedin.com/in/hmziqrs".to_string(),
                icon: rsx!{ Icon { icon: LdGlobe } },
                // icon: SocialLinkIcon::Linkedin,
                label: "linkedin/hmziqrs".to_string(),
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
                        // let icon_component = match link.icon {
                        //     SocialLinkIcon::Gmail => rsx! { Icon { icon: SiGmail {} } },
                        //     SocialLinkIcon::Telegram => rsx! { Icon { icon: SiTelegram {} } },
                        //     SocialLinkIcon::X => rsx! { Icon { icon: SiX {} } },
                        //     SocialLinkIcon::Linkedin => rsx! { Icon { icon: LdGlobe {} } },
                        // };
                        rsx! {
                            a {
                                key: "{link.href}",
                                href: "{link.href}",
                                target: "{target}",
                                class: "flex items-center gap-3 px-6 py-3 bg-zinc-200 dark:bg-zinc-800 hover:bg-zinc-300 dark:hover:bg-zinc-700 text-zinc-800 dark:text-zinc-200 rounded-md transition-colors duration-300",
                                div {
                                    class: "w-4",
                                    // class: "[&>svg]:h-2 [&>svg]:w-2",
                                    {link.icon.clone()}
                                }
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
