use dioxus::{document::Style, prelude::*};

#[derive(Props, PartialEq, Clone)]
pub struct AppSpacerProps {
    #[props(default = false)]
    break_after: bool,
}

pub fn AppSpacer(props: AppSpacerProps) -> Element {
    rsx! {
        section {
            class: format!("h-16 {}", if props.break_after { "print:h-12" } else { "print:h-80" }),
            style: if props.break_after { "page-break-after: always" },
        }
    }
}
