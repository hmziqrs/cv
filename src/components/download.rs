use dioxus::{html::col::span, prelude::*};
use hmziq_dioxus_free_icons::{
    icons::ld_icons::{LdFileImage, LdFileText},
    Icon,
};

struct Download {
    label: String,
    filename: String,
    icon: Element,
}

fn get_download() -> Vec<Download> {
    vec![
        Download {
            label: "PDF (light mode)".to_string(),
            filename: "/hmziqrs-light-cv.pdf".to_string(),
            icon: rsx! { Icon{ icon: LdFileText }},
        },
        Download {
            label: "PDF (dark mode)".to_string(),
            filename: "/hmziqrs-dark-cv.pdf".to_string(),
            icon: rsx! { Icon{ icon: LdFileText }},
        },
        Download {
            label: "JPEG (light mode)".to_string(),
            filename: "/hmziqrs-light-cv.pdf".to_string(),
            icon: rsx! { Icon{ icon: LdFileImage }},
        },
        Download {
            label: "JPEG (dark mode)".to_string(),
            filename: "/hmziqrs-dark-cv.pdf".to_string(),
            icon: rsx! { Icon{ icon: LdFileImage }},
        },
    ]
}

pub fn AppDownload() -> Element {
    rsx! {
        div {
            class: "flex bg-zinc-100 dark:bg-zinc-900 print:hidden jpeg",
            div {
                class: "container max-w-8xl mx-auto px-6 py-10",
                h2 {
                    class: "text-2xl font-semibold text-zinc-800 dark:text-zinc-100 mb-8",
                    "Download CV"
                }

                div {
                    class: "flex flex-row flex-wrap gap-4",
                    {
                        get_download().iter().map(|download| {
                            rsx! {
                                a {
                                    key: "{download.filename}",
                                    href: "/files{download.filename}",
                                    download: true,
                                    class: "flex items-center justify-center
                                        sm:gap-3 gap-2 sm:px-6 px-4 sm:py-3 py-2
                                        bg-zinc-200 dark:bg-zinc-800
                                        hover:bg-zinc-300 dark:hover:bg-zinc-700
                                        text-zinc-800 dark:text-zinc-200
                                        rounded-md transition-colors duration-300",

                                    // Empty div instead of an icon
                                    span {
                                        class: "sm:w-4 w-3.5",
                                        {download.icon.clone()}
                                    }

                                    span {
                                        class: "font-medium sm:text-sm text-xs",
                                        "{download.label}"
                                    }
                                }
                            }
                        })
                    }
                }
            }
        }
    }
}
