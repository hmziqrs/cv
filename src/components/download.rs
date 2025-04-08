use dioxus::prelude::*;

const DOWNLOADS: &[Download] = &[
    Download {
        label: "Light PDF",
        filename: "/hmziqrs-light-cv.pdf",
    },
    Download {
        label: "Dark PDF",
        filename: "/hmziqrs-dark-cv.pdf",
    },
    Download {
        label: "Light JPEG",
        filename: "/hmziqrs-light-cv.jpg",
    },
    Download {
        label: "Dark JPEG",
        filename: "/hmziqrs-dark-cv.jpg",
    },
];

struct Download {
    label: &'static str,
    filename: &'static str,
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
                        DOWNLOADS.iter().map(|download| {
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
                                    div {
                                        class: "sm:w-4 w-3.5"
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
