use dioxus::prelude::*;

pub struct Skill {
    pub label: String,
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
                },
                Skill {
                    label: "Next JS".to_string(),
                },
                Skill {
                    label: "Tailwind CSS".to_string(),
                },
                Skill {
                    label: "Webassembly".to_string(),
                },
                Skill {
                    label: "Typescript".to_string(),
                },
                Skill {
                    label: "Material UI".to_string(),
                },
                Skill {
                    label: "Firebase".to_string(),
                },
            ],
        },
        SkillSection {
            key: "Mobile".to_string(),
            skills: vec![
                Skill {
                    label: "Flutter".to_string(),
                },
                Skill {
                    label: "React Native".to_string(),
                },
                Skill {
                    label: "Socket.IO".to_string(),
                },
                Skill {
                    label: "Internationalization".to_string(),
                },
                Skill {
                    label: "Multi Threaded Apps".to_string(),
                },
                Skill {
                    label: "E2E Testing".to_string(),
                },
                Skill {
                    label: "Pixel Perfect UI".to_string(),
                },
                Skill {
                    label: "Parallax Animations".to_string(),
                },
            ],
        },
        SkillSection {
            key: "Server".to_string(),
            skills: vec![
                Skill {
                    label: "Rust".to_string(),
                },
                Skill {
                    label: "Node JS".to_string(),
                },
                Skill {
                    label: "Adonis JS".to_string(),
                },
                Skill {
                    label: "Express JS".to_string(),
                },
                Skill {
                    label: "CLI App".to_string(),
                },
                Skill {
                    label: "Github Actions".to_string(),
                },
                Skill {
                    label: "Postgres".to_string(),
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

                                                // Empty div instead of icon
                                                div { class: "text-zinc-700 dark:text-zinc-300 sm:w-4 w-4" }

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
