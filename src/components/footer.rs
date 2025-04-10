use chrono::{Datelike, Local};
use dioxus::prelude::*;
use hmziq_dioxus_free_icons::icons::ld_icons::LdGlobe;
use hmziq_dioxus_free_icons::icons::si_icons::{
    SiBehance,
    SiCodepen,
    SiDevdotto,
    SiDribbble,
    SiFacebook,
    SiGithub,
    SiHashnode,
    // Hidden social media icons
    SiInstagram,
    SiMedium,
    SiReddit,
    SiStackoverflow,
    SiTiktok,
    SiTwitch,
    SiX,
    SiYoutube,
};
use hmziq_dioxus_free_icons::{Icon, IconShape};

pub struct SocialLink {
    pub href: String,
    pub icon: Element,
    pub label: String,
}

pub fn get_visible_social_links() -> Vec<SocialLink> {
    vec![
        SocialLink {
            href: "https://hmziq.rs".to_string(),
            icon: rsx! { Icon { icon: LdGlobe } },
            label: "Website".to_string(),
        },
        SocialLink {
            href: "https://github.com/hmziqrs".to_string(),
            icon: rsx! { Icon { icon: SiGithub } },
            label: "Github".to_string(),
        },
        SocialLink {
            href: "https://x.com/hmziqrs".to_string(),
            icon: rsx! { Icon { icon: SiX } },
            label: "X (Twitter)".to_string(),
        },
        SocialLink {
            href: "https://linkedin.com/in/hmziqrs".to_string(),
            icon: rsx! { Icon { icon: LdGlobe } },
            label: "LinkedIn".to_string(),
        },
    ]
}

pub fn get_hidden_social_links() -> Vec<SocialLink> {
    vec![
        SocialLink {
            href: "https://instagram.com/hmziqrs".to_string(),
            icon: rsx! { Icon { icon: SiInstagram } },
            label: "Instagram".to_string(),
        },
        SocialLink {
            href: "https://youtube.com/@hmziqrs".to_string(),
            icon: rsx! { Icon { icon: SiYoutube } },
            label: "YouTube".to_string(),
        },
        SocialLink {
            href: "https://twitch.tv/hmziqrs".to_string(),
            icon: rsx! { Icon { icon: SiTwitch } },
            label: "Twitch".to_string(),
        },
        SocialLink {
            href: "https://tiktok.com/@hmziqrs".to_string(),
            icon: rsx! { Icon { icon: SiTiktok } },
            label: "TikTok".to_string(),
        },
        SocialLink {
            href: "https://reddit.com/user/hmziqrs".to_string(),
            icon: rsx! { Icon { icon: SiReddit } },
            label: "Reddit".to_string(),
        },
        SocialLink {
            href: "https://facebook.com/hmziqrs".to_string(),
            icon: rsx! { Icon { icon: SiFacebook } },
            label: "Facebook".to_string(),
        },
        SocialLink {
            href: "https://medium.com/@hmziqrs".to_string(),
            icon: rsx! { Icon { icon: SiMedium } },
            label: "Medium".to_string(),
        },
        SocialLink {
            href: "https://dev.to/hmziqrs".to_string(),
            icon: rsx! { Icon { icon: SiDevdotto } },
            label: "Dev.to".to_string(),
        },
        SocialLink {
            href: "https://hashnode.com/@hmziqrs".to_string(),
            icon: rsx! { Icon { icon: SiHashnode } },
            label: "Hashnode".to_string(),
        },
        SocialLink {
            href: "https://stackoverflow.com/users/6908566/hmziqrs".to_string(),
            icon: rsx! { Icon { icon: SiStackoverflow } },
            label: "Stack Overflow".to_string(),
        },
        SocialLink {
            href: "https://codepen.io/hmziqrs".to_string(),
            icon: rsx! { Icon { icon: SiCodepen } },
            label: "CodePen".to_string(),
        },
        SocialLink {
            href: "https://dribbble.com/hmziqrs".to_string(),
            icon: rsx! { Icon { icon: SiDribbble } },
            label: "Dribbble".to_string(),
        },
        SocialLink {
            href: "https://behance.net/hmziqrs".to_string(),
            icon: rsx! { Icon { icon: SiBehance } },
            label: "Behance".to_string(),
        },
    ]
}

pub fn get_formatted_date() -> String {
    // For simplicity, we're using a static approach
    // In a real-world app, you might want to use a server function or other approach
    let now = Local::now();
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    format!(
        "{} {}, {}",
        months[(now.month() as usize) - 1],
        now.day(),
        now.year()
    )
}

pub fn AppFooter() -> Element {
    let visible_links = get_visible_social_links();
    let hidden_links = get_hidden_social_links();
    let current_year = Local::now().year();
    let last_updated = get_formatted_date();

    rsx! {
        footer { class: "flex bg-zinc-100 dark:bg-zinc-900 print:hidden jpeg",
            div { class: "container max-w-8xl mx-auto px-6 py-8",
                div { class: "flex flex-col items-center gap-4",
                    // Visible Social Links
                    div { class: "flex gap-4",
                        {visible_links.iter().map(|link| {
                            rsx! {
                                a {
                                    key: "{link.href}",
                                    href: "{link.href}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "text-zinc-600 dark:text-zinc-100
                                    hover:text-zinc-900 dark:hover:text-zinc-100
                                    transition-colors duration-300",
                                    aria_label: "{link.label}",
                                    div { class: "w-5 h-5",
                                        {link.icon.clone()}
                                    }
                                }
                            }
                        })}
                    }

                    // Hidden Social Links
                    div { class: "sr-only",
                        {hidden_links.iter().map(|link| {
                            rsx! {
                                a {
                                    key: "{link.href}",
                                    href: "{link.href}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    aria_label: "{link.label}",
                                    div { class: "w-5 h-5",
                                        {link.icon.clone()}
                                    }
                                }
                            }
                        })}
                    }

                    // Copyright
                    div { class: "text-sm font-mono text-center text-zinc-600 dark:text-zinc-100",
                        "© {current_year} Hamza Iqbal. All rights reserved."
                    }
                    div { class: "text-xs font-mono text-center text-zinc-600 dark:text-zinc-100",
                        "Updated on {last_updated}"
                    }
                }
            }
        }
    }
}
