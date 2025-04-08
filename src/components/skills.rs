use dioxus::prelude::*;
use hmziq_dioxus_free_icons::icons::ld_icons::{
    LdCpu, LdDna, LdEllipsis, LdFileTerminal, LdFlaskConical, LdGlobe, LdPlay,
};
use hmziq_dioxus_free_icons::icons::si_icons::{
    SiAdonisjs, SiExpress, SiFirebase, SiFlutter, SiGithubactions, SiMaterialdesign, SiNextdotjs,
    SiNodedotjs, SiPostgresql, SiReact, SiRust, SiSocketdotio, SiTailwindcss, SiTypescript,
    SiWebassembly,
};
use hmziq_dioxus_free_icons::Icon;

pub struct Skill {
    pub label: String,
    pub icon: Element,
}

pub struct SkillSection {
    pub key: String,
    pub skills: Vec<Skill>,
}

pub fn get_skills() -> Vec<SkillSection> {
    vec![
        SkillSection {
            key: "Web".to_string(),
            skills: vec![
                Skill {
                    label: "React JS".to_string(),
                    icon: rsx! { Icon { icon: SiReact } },
                },
                Skill {
                    label: "Next JS".to_string(),
                    icon: rsx! { Icon { icon: SiNextdotjs } },
                },
                Skill {
                    label: "Tailwind CSS".to_string(),
                    icon: rsx! { Icon { icon: SiTailwindcss } },
                },
                Skill {
                    label: "Webassembly".to_string(),
                    icon: rsx! { Icon { icon: SiWebassembly } },
                },
                Skill {
                    label: "Typescript".to_string(),
                    icon: rsx! { Icon { icon: SiTypescript } },
                },
                Skill {
                    label: "Material UI".to_string(),
                    icon: rsx! { Icon { icon: SiMaterialdesign } },
                },
                Skill {
                    label: "Firebase".to_string(),
                    icon: rsx! { Icon { icon: SiFirebase } },
                },
            ],
        },
        SkillSection {
            key: "Mobile".to_string(),
            skills: vec![
                Skill {
                    label: "Dioxus".to_string(),
                    icon: rsx! { Icon { icon: LdDna } },
                },
                Skill {
                    label: "Flutter".to_string(),
                    icon: rsx! { Icon { icon: SiFlutter } },
                },
                Skill {
                    label: "React Native".to_string(),
                    icon: rsx! { Icon { icon: SiReact } },
                },
                Skill {
                    label: "Socket.IO".to_string(),
                    icon: rsx! { Icon { icon: SiSocketdotio } },
                },
                Skill {
                    label: "Internationalization".to_string(),
                    icon: rsx! { Icon { icon: LdGlobe } },
                },
                Skill {
                    label: "Multi Threaded Apps".to_string(),
                    icon: rsx! { Icon { icon: LdCpu } },
                },
                Skill {
                    label: "E2E Testing".to_string(),
                    icon: rsx! { Icon { icon: LdFlaskConical } },
                },
                Skill {
                    label: "Pixel Perfect UI".to_string(),
                    icon: rsx! { Icon { icon: LdEllipsis } },
                },
                Skill {
                    label: "Parallax Animations".to_string(),
                    icon: rsx! { Icon { icon: LdPlay } },
                },
            ],
        },
        SkillSection {
            key: "Server".to_string(),
            skills: vec![
                Skill {
                    label: "Rust".to_string(),
                    icon: rsx! { Icon { icon: SiRust } },
                },
                Skill {
                    label: "Axum".to_string(),
                    icon: rsx! { Icon { icon: SiRust } },
                },
                Skill {
                    label: "Node JS".to_string(),
                    icon: rsx! { Icon { icon: SiNodedotjs } },
                },
                Skill {
                    label: "Adonis JS".to_string(),
                    icon: rsx! { Icon { icon: SiAdonisjs } },
                },
                Skill {
                    label: "Express JS".to_string(),
                    icon: rsx! { Icon { icon: SiExpress } },
                },
                Skill {
                    label: "CLI App".to_string(),
                    icon: rsx! { Icon { icon: LdFileTerminal } },
                },
                Skill {
                    label: "Github Actions".to_string(),
                    icon: rsx! { Icon { icon: SiGithubactions } },
                },
                Skill {
                    label: "Postgres".to_string(),
                    icon: rsx! { Icon { icon: SiPostgresql } },
                },
            ],
        },
    ]
}

pub fn AppSkills() -> Element {
    let skills = get_skills();

    rsx! {
        div { class: "flex bg-zinc-100 dark:bg-zinc-900",
            div { class: "container max-w-8xl mx-auto px-6 py-10",
                h2 { class: "text-2xl font-semibold text-zinc-800 dark:text-zinc-100 mb-8",
                    "Skills"
                }

                div { class: "flex flex-col gap-10",
                    {skills.iter().map(|section| {
                        rsx! {
                            div { key: "{section.key}",
                                // Hidden section label for semantic HTML
                                span { class: "sr-only",
                                    "{section.key}"
                                }

                                div { class: "flex flex-wrap gap-3",
                                    {section.skills.iter().map(|skill| {
                                        rsx! {
                                            div {
                                                key: "{skill.label}",
                                                class: "flex items-center sm:gap-3 gap-2 cursor-pointer py-2 px-4 bg-zinc-200 dark:bg-zinc-800 hover:bg-zinc-300 dark:hover:bg-zinc-700 rounded-full duration-300",

                                                // Icon component
                                                div { class: "text-zinc-700 dark:text-zinc-300 w-4",
                                                    {skill.icon.clone()}
                                                }

                                                p { class: "text-xs sm:text-sm font-medium font-mono text-zinc-800 dark:text-zinc-200",
                                                    "{skill.label}"
                                                }
                                            }
                                        }
                                    })}
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}
