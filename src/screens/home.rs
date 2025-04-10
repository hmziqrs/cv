use dioxus::prelude::*;

use crate::components::{
    AppDownload, AppExperience, AppFooter, AppHeader, AppProject, AppSkills, AppSpacer,
};

#[component]
pub fn HomeScreen() -> Element {
    rsx! {
        main {
            class: "h-full",
            role: "main",
            aria_label: "Portfolio content",
            AppHeader {}
        }
        AppSpacer {}
        section {
            id: "skills",
            aria_label: "Skills section",
            class: "skills-sections",
            AppSkills {}
        }
        AppSpacer { jpeg_mode: true }
        section {
            id: "download",
            aria_label: "Download section",
            class: "download-section",
            AppDownload {}
        }
        AppSpacer { break_after: true }
        AppProject {}
        AppSpacer { break_after: true }
        AppExperience {}
        AppSpacer { jpeg_mode: true }
        AppFooter {}
    }
}
