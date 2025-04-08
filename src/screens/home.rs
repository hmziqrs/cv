use dioxus::prelude::*;

use crate::components::AppHeader;

#[component]
pub fn HomeScreen() -> Element {
    rsx! {
        div {
            div {
                AppHeader {}
            }
        }
    }
}
