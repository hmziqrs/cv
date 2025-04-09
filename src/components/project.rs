use dioxus::prelude::*;
use hmziq_dioxus_free_icons::icons::ld_icons::{
    LdBox, LdGlobe, LdLayoutGrid, LdMonitorSmartphone, LdSmartphone, LdTerminal,
};
use hmziq_dioxus_free_icons::icons::si_icons::{SiAppstore, SiGithub, SiGoogleplay};
use hmziq_dioxus_free_icons::{Icon, IconShape};

// Project types and data structures
pub struct Project {
    pub name: String,
    pub contribution: String,
    pub project_type: String,
    pub description: Vec<String>,
    pub thumbnail: Asset,
    pub buttons: Vec<ProjectButton>,
}

pub struct ProjectButton {
    pub label: String,
    pub icon: Element,
    pub link: String,
}

// Project data
pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            name: "Flutter UI Designs".to_string(),
            contribution: "Mobile App".to_string(),
            project_type: "Open Source".to_string(),
            description: vec![
                "Features complex parallax animations. ".to_string(),
                "Desktop & web app support. ".to_string(),
                "Responsive UI. ".to_string(),
                "Basic internationalization. ".to_string(),
                "Light and Dark theme support.".to_string(),
            ],
            thumbnail: asset!("/assets/projects/flutter-ui-designs.jpg"),
            buttons: vec![
                ProjectButton {
                    label: "Google Play".to_string(),
                    icon: rsx!{ Icon { icon: SiGoogleplay } },
                    link: "https://play.google.com/store/apps/details?id=com.onemdev.flutter_ui_challenges".to_string(),
                },
                ProjectButton {
                    label: "Github".to_string(),
                    icon: rsx!{ Icon { icon: SiGithub } },
                    link: "https://github.com/hmziqrs/flutter-ui-designs".to_string(),
                },
                ProjectButton {
                    label: "Web".to_string(),
                    icon: rsx!{ Icon { icon: LdGlobe } },
                    link: "https://flutter-uis.hmziq.xyz/".to_string(),
                },
            ],
        },
        Project {
            name: "Flutter movie concept".to_string(),
            contribution: "Mobile App".to_string(),
            project_type: "Open Source".to_string(),
            thumbnail: asset!("/assets/projects/movie-concept.jpg"),
            description: vec![
                "Features complex parallax animations.".to_string(),
                "Scales on desktop, tablets & browser.".to_string(),
            ],
            buttons: vec![
                ProjectButton {
                    label: "Google Play".to_string(),
                    icon: rsx!{ Icon { icon: SiGoogleplay } },
                    link: "https://play.google.com/store/apps/details?id=com.onemdev.invmovieconcept1".to_string(),
                },
                ProjectButton {
                    label: "Github".to_string(),
                    icon: rsx!{ Icon { icon: SiGithub } },
                    link: "https://github.com/hmziqrs/invmovieconcept1".to_string(),
                },
                ProjectButton {
                    label: "Web".to_string(),
                    icon: rsx!{ Icon { icon: LdGlobe } },
                    link: "https://movieui.hmziq.xyz".to_string(),
                },
            ],
        },
        Project {
            name: "React Native Loop".to_string(),
            contribution: "Mobile App".to_string(),
            project_type: "Open Source".to_string(),
            thumbnail: asset!("/assets/projects/react-native-loop.jpg"),
            description: vec![
                "Features complex parallax animations.".to_string(),
                "Scales on desktop, tablets & browser.".to_string(),
            ],
            buttons: vec![
                ProjectButton {
                    label: "Google Play".to_string(),
                    icon: rsx!{ Icon { icon: SiGoogleplay } },
                    link: "https://play.google.com/store/apps/details?id=com.onemdev.rnloop".to_string(),
                },
                ProjectButton {
                    label: "Github".to_string(),
                    icon: rsx!{ Icon { icon: SiGithub } },
                    link: "https://github.com/hmziqrs/react-native-loop-game".to_string(),
                },
                ProjectButton {
                    label: "Web".to_string(),
                    icon: rsx!{ Icon { icon: LdGlobe } },
                    link: "https://rnloop.hmziq.xyz".to_string(),
                },
            ],
        },
        Project {
            name: "Gandalf".to_string(),
            contribution: "Mobile App".to_string(),
            project_type: "Open Source".to_string(),
            thumbnail: asset!("/assets/projects/gandalf.jpg"),
            description: vec![],
            buttons: vec![
                ProjectButton {
                    label: "Google Play".to_string(),
                    icon: rsx!{ Icon { icon: SiGoogleplay } },
                    link: "https://play.google.com/store/apps/details?id=com.onemdev.gandalf".to_string(),
                },
                ProjectButton {
                    label: "Github".to_string(),
                    icon: rsx!{ Icon { icon: SiGithub } },
                    link: "https://github.com/hmziqrs/gandlaf-sax".to_string(),
                },
                ProjectButton {
                    label: "Web".to_string(),
                    icon: rsx!{ Icon { icon: LdGlobe } },
                    link: "https://gandalf.hmziq.xyz".to_string(),
                },
            ],
        },
        Project {
            name: "Golang minesweeper".to_string(),
            contribution: "CLI App".to_string(),
            project_type: "Open Source".to_string(),
            thumbnail: asset!("/assets/projects/golang-minesweeper.gif"),
            description: vec![
                "Features complex parallax animations.".to_string(),
                "Scales on desktop, tablets & browser.".to_string(),
            ],
            buttons: vec![
                ProjectButton {
                    label: "Github".to_string(),
                    icon: rsx!{ Icon { icon: SiGithub } },
                    link: "https://github.com/hmziqrs/go-minesweeper".to_string(),
                },
            ],
        },
        Project {
            name: "My CV".to_string(),
            contribution: "Web App".to_string(),
            project_type: "Open Source".to_string(),
            description: vec![
                "Features complex parallax animations.".to_string(),
                "Scales on desktop, tablets & browser.".to_string(),
            ],
            thumbnail: asset!("/assets/projects/my-cv.jpg"),
            buttons: vec![
                ProjectButton {
                    label: "Github".to_string(),
                    icon: rsx!{ Icon { icon: SiGithub } },
                    link: "https://github.com/hmziqrs/cv".to_string(),
                },
                ProjectButton {
                    label: "Web".to_string(),
                    icon: rsx!{ Icon { icon: LdGlobe } },
                    link: "https://cv.hmziq.rs".to_string(),
                },
            ],
        },
        Project {
            name: "Wheelbees".to_string(),
            contribution: "Full Stack".to_string(),
            project_type: "Product".to_string(),
            thumbnail: asset!("/assets/projects/wheelbees.jpg"),
            description: vec![],
            buttons: vec![
                ProjectButton {
                    label: "Google Play".to_string(),
                    icon: rsx!{ Icon { icon: SiGoogleplay } },
                    link: "https://play.google.com/store/apps/details?id=com.wheelbees".to_string(),
                },
                ProjectButton {
                    label: "App Store".to_string(),
                    icon: rsx!{ Icon { icon: SiAppstore } },
                    link: "https://apps.apple.com/nl/app/wheelbees/id1585504248".to_string(),
                },
                ProjectButton {
                    label: "Web".to_string(),
                    icon: rsx!{ Icon { icon: LdGlobe } },
                    link: "https://wheelbees.com".to_string(),
                },
            ],
        },
        Project {
            name: "QuestSocial".to_string(),
            contribution: "Full Stack".to_string(),
            project_type: "Product".to_string(),
            thumbnail: asset!("/assets/projects/quest.jpg"),
            description: vec![],
            buttons: vec![
                ProjectButton {
                    label: "App Store".to_string(),
                    icon: rsx!{ Icon { icon: SiAppstore } },
                    link: "https://apps.apple.com/us/app/questsocial/id1529478932".to_string(),
                },
            ],
        },
        Project {
            name: "Mixfame".to_string(),
            contribution: "Mobile App".to_string(),
            project_type: "Product".to_string(),
            thumbnail: asset!("/assets/projects/mixfame.jpg"),
            description: vec![],
            buttons: vec![
                ProjectButton {
                    label: "Google Play".to_string(),
                    icon: rsx!{ Icon { icon: SiGoogleplay } }, // Fixed from SiAppstore to SiGoogleplay
                    link: "https://play.google.com/store/apps/details?id=com.mobile.mixfame".to_string(),
                },
                ProjectButton {
                    label: "App Store".to_string(),
                    icon: rsx!{ Icon { icon: SiAppstore } },
                    link: "https://apps.apple.com/us/app/mixfame/id1586985916".to_string(),
                },
                ProjectButton {
                    label: "Web".to_string(),
                    icon: rsx!{ Icon { icon: LdGlobe } },
                    link: "https://mixfame.com".to_string(),
                },
            ],
        },
        Project {
            name: "Peekaboo Guru".to_string(),
            contribution: "Full Stack".to_string(),
            project_type: "Product".to_string(),
            thumbnail: asset!("/assets/projects/peekaboo-guru.webp"),
            description: vec![],
            buttons: vec![
                ProjectButton {
                    label: "Google Play".to_string(),
                    icon: rsx!{ Icon { icon: SiGoogleplay } },
                    link: "https://play.google.com/store/search?q=peekaboo%20guru&c=apps".to_string(),
                },
                ProjectButton {
                    label: "App Store".to_string(),
                    icon: rsx!{ Icon { icon: SiAppstore } },
                    link: "https://apps.apple.com/pk/app/peekaboo-guru/id1114129707".to_string(),
                },
                ProjectButton {
                    label: "Web".to_string(),
                    icon: rsx!{ Icon { icon: LdGlobe } },
                    link: "https://peekaboo.guru/".to_string(),
                },
            ],
        },
        Project {
            name: "Sastaticket.pk".to_string(),
            contribution: "Mobile App".to_string(),
            project_type: "Product".to_string(),
            description: vec![
                "Features complex parallax animations.".to_string(),
                "Scales on desktop, tablets & browser.".to_string(),
            ],
            thumbnail: asset!("/assets/projects/sasta-ticket.jpg"),
            buttons: vec![
                ProjectButton {
                    label: "Google Play".to_string(),
                    icon: rsx!{ Icon { icon: SiGoogleplay } },
                    link: "https://play.google.com/store/apps/details?id=com.pk.sastaticket".to_string(),
                },
                ProjectButton {
                    label: "App Store".to_string(),
                    icon: rsx!{ Icon { icon: SiAppstore } },
                    link: "https://apps.apple.com/by/app/sastaticket-flight-hotels/id1564441908".to_string(),
                },
                ProjectButton {
                    label: "Web".to_string(),
                    icon: rsx!{ Icon { icon: LdGlobe } },
                    link: "https://www.sastaticket.pk/".to_string(),
                },
            ],
        },
        Project {
            name: "Grow youth ministry".to_string(),
            contribution: "Mobile App".to_string(),
            project_type: "Product".to_string(),
            description: vec![
                "Features complex parallax animations.".to_string(),
                "Scales on desktop, tablets & browser.".to_string(),
            ],
            thumbnail: asset!("/assets/projects/grow-more.jpg"),
            buttons: vec![
                ProjectButton {
                    label: "Google Play".to_string(),
                    icon: rsx!{ Icon { icon: SiGoogleplay } },
                    link: "https://play.google.com/store/apps/details?id=com.growappv2".to_string(),
                },
                ProjectButton {
                    label: "App Store".to_string(),
                    icon: rsx!{ Icon { icon: SiAppstore } },
                    link: "https://apps.apple.com/us/app/grow-youth-kids-ministry/id1260816727".to_string(),
                },
            ],
        },
    ]
}

fn ProjectTypeBadge(project_type: String) -> Element {
    if project_type == "Product" {
        return rsx! {
            div {
                class: "flex items-center gap-1.5 text-xs px-2 py-0.5
                bg-blue-100 dark:bg-blue-950
                text-blue-700 dark:text-blue-200
                border border-blue-700/30 dark:border-blue-200/30
                rounded-full font-medium
                hover:bg-blue-200 dark:hover:bg-blue-900
                transition-colors duration-200",
                div {
                    class: "w-3.5",
                    Icon { icon: LdBox }
                }
                "{project_type}"
            }
        };
    }

    rsx! {
        div {
            class: "flex items-center gap-1.5 text-xs px-2 py-0.5
            bg-green-100 dark:bg-green-950
            text-green-700 dark:text-green-200
            border border-green-700/30 dark:border-green-200/30
            rounded-full font-medium
            hover:bg-green-200 dark:hover:bg-green-900
            transition-colors duration-200",
            div {
                class: "w-3.5",
                Icon { icon: LdGlobe }
            }
            "{project_type}"
        }
    }
}

fn ContributionBadge(role: String) -> Element {
    let (colors, icon_element) = match role.as_str() {
        "Mobile App" => (
            "bg-purple-100 dark:bg-purple-950 text-purple-700 dark:text-purple-300 border-purple-700/30 dark:border-purple-300/30 hover:bg-purple-200 dark:hover:bg-purple-900",
            rsx!{ Icon { icon: LdSmartphone } }
        ),
        "Full Stack" => (
            "bg-red-100 dark:bg-red-950 text-red-700 dark:text-red-300 border-red-700/30 dark:border-red-300/30 hover:bg-red-200 dark:hover:bg-red-900",
            rsx!{ Icon { icon: LdMonitorSmartphone } }
        ),
        "Web App" => (
            "bg-orange-100 dark:bg-orange-950 text-orange-700 dark:text-orange-300 border-orange-700/30 dark:border-orange-300/30 hover:bg-orange-200 dark:hover:bg-orange-900",
            rsx!{ Icon { icon: LdLayoutGrid } }
        ),
        "CLI App" => (
            "bg-teal-100 dark:bg-teal-950 text-teal-700 dark:text-teal-300 border-teal-700/30 dark:border-teal-300/30 hover:bg-teal-200 dark:hover:bg-teal-900",
            rsx!{ Icon { icon: LdTerminal } }
        ),
        _ => (
            "bg-gray-100 dark:bg-gray-950 text-gray-700 dark:text-gray-300 border-gray-700/30 dark:border-gray-300/30 hover:bg-gray-200 dark:hover:bg-gray-900",
            rsx!{ Icon { icon: LdBox } }
        ),
    };

    rsx! {
        div {
            class: "flex items-center gap-1.5 text-xs px-2 py-0.5
            rounded-full font-medium border
            transition-colors duration-200
            {colors}",
            div {
                class: "w-3.5",
                {icon_element}
            }
            span {
                "{role}"
            }
        }
    }
}

pub fn AppProject() -> Element {
    let projects = get_projects();

    rsx! {
        div { class: "flex bg-zinc-100 dark:bg-zinc-900",
            div { class: "container max-w-8xl mx-auto px-6 py-10",
                h2 { class: "text-2xl font-semibold text-zinc-800 dark:text-zinc-100 mb-8",
                    "Projects"
                }

                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                    {projects.iter().map(|project| {
                        rsx! {
                            div {
                                key: "{project.name}",
                                class: "flex flex-col bg-zinc-200 dark:bg-zinc-800 rounded-lg overflow-hidden
                                group hover:transform hover:scale-[1.02] transition-all duration-300
                                border border-transparent hover:border-zinc-300 dark:hover:border-zinc-700",

                                // Project Image
                                div { class: "relative h-48 w-full overflow-hidden",
                                    img {
                                        src: "{project.thumbnail}",
                                        alt: "{project.name}",
                                        class: "object-cover group-hover:scale-105 transition-transform duration-300"
                                    }
                                }

                                // Project Details
                                div { class: "p-4 flex flex-col flex-grow",
                                    div { class: "flex justify-between items-start mb-2",
                                        h3 { class: "text-lg font-semibold text-zinc-800 dark:text-zinc-100",
                                            "{project.name}"
                                        }
                                    }

                                    div { class: "flex flex-wrap items-center gap-2 mb-4",
                                        {ProjectTypeBadge(project.project_type.clone())}
                                        {ContributionBadge(project.contribution.clone())}
                                    }

                                    ul { class: "text-sm text-zinc-600 dark:text-zinc-400 mb-4 flex-grow sr-only",
                                        {project.description.iter().map(|desc| {
                                            rsx! {
                                                li { class: "mb-1",
                                                    "• {desc}"
                                                }
                                            }
                                        })}
                                    }

                                    // Project Links
                                    div { class: "flex flex-wrap gap-2 mb-auto",
                                        {project.buttons.iter().map(|button| {
                                            rsx! {
                                                a {
                                                    key: "{button.link}",
                                                    href: "{button.link}",
                                                    target: "_blank",
                                                    class: "flex items-center gap-2 px-3 py-2
                                                    bg-zinc-300 dark:bg-zinc-700
                                                    hover:bg-zinc-400 dark:hover:bg-zinc-600
                                                    rounded-md text-sm font-medium
                                                    text-zinc-800 dark:text-zinc-200
                                                    transition-colors duration-300",
                                                    div { class: "w-4 h-4",
                                                        {button.icon.clone()}
                                                    }
                                                    "{button.label}"
                                                }
                                            }
                                        })}
                                    }
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}
