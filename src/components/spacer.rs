use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct AppSpacerProps {
    #[props(default = false)]
    break_after: bool,
    #[props(default = false)]
    jpeg_mode: bool,
}

#[component]
pub fn AppSpacer(props: AppSpacerProps) -> Element {
    rsx! {
        section {
            class: format!("h-16 {} {}", if props.break_after { "print:h-12" } else { "print:h-80" }, if props.jpeg_mode { "jpeg" } else { "" }),
            style: if props.break_after { "page-break-after: always" },
        }
    }
}
