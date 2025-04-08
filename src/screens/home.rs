use dioxus::prelude::*;

use crate::components::{AppDownload, AppHeader, AppSkills};

#[component]
pub fn HomeScreen() -> Element {
    rsx! {
        main {
            class: "h-full",
            role: "main",
            aria_label: "Portfolio content",
            AppHeader {}
        }
        SectionSpacer {}
        section {
            id: "skills",
            aria_label: "Skills section",
            class: "skills-sections",
            AppSkills {}
        }
        SectionSpacer {}
        section {
            id: "download",
            aria_label: "Download section",
            class: "download-section",
            AppDownload {}
        }
        SectionSpacer {}
    }
}

#[component]
fn SectionSpacer() -> Element {
    rsx! {
        section {
            class: "h-16 print:h-12"
        }
    }
}

fn Spacer() -> Element {
    rsx! {
        section {
            class: "h-16 print:h-12"
        }
    }
}
