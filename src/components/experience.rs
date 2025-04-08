use dioxus::prelude::*;
use hmziq_dioxus_free_icons::icons::ld_icons::{
    LdBell, LdCreditCard, LdFileCode, LdPieChart, LdPlay,
};
use hmziq_dioxus_free_icons::icons::si_icons::{
    SiAdonisjs, SiAmazon, SiAngular, SiAsana, SiDropbox, SiEthereum, SiEthers, SiFirebase,
    SiFlutter, SiGo, SiGooglechrome, SiGoogledrive, SiMaterialdesign, SiMysql, SiNodedotjs,
    SiPostgresql, SiReact, SiRedux, SiStyledcomponents, SiTypescript, SiWebpack,
};
use hmziq_dioxus_free_icons::{Icon, IconShape};
use std::collections::HashMap;

pub struct TechItem {
    pub label: String,
    pub icon: Element,
}

pub struct Position {
    pub position: String,
    pub date: String,
}

pub struct ExperienceItem {
    pub journey: Vec<Position>,
    pub company: String,
    pub contract: String,
    pub points: Vec<String>,
    pub tech: Vec<TechItem>,
}

pub fn get_experience() -> Vec<ExperienceItem> {
    vec![
        ExperienceItem {
            journey: vec![
                Position {
                    position: "Freelance mobile engineer".to_string(),
                    date: "Dec 2023 - Jun 2024".to_string(),
                },
            ],
            company: "Mixfame".to_string(),
            contract: "Freelance".to_string(),
            points: vec![
                "Built Flutter mobile app from scratch for Mixfame, a talent management platform, integrating Firebase, and In app purchase.".to_string(),
                "Deeplinking via push notifications and dynamic links for user engagement and retention.".to_string(),
            ],
            tech: vec![
                TechItem {
                    label: "Flutter".to_string(),
                    icon: rsx!{ Icon { icon: SiFlutter } }
                },
                TechItem {
                    label: "Bloc".to_string(),
                    icon: rsx!{ Icon { icon: LdFileCode } }
                },
                TechItem {
                    label: "Provider".to_string(),
                    icon: rsx!{ Icon { icon: LdFileCode } }
                },
                TechItem {
                    label: "Firebase".to_string(),
                    icon: rsx!{ Icon { icon: SiFirebase } }
                },
                TechItem {
                    label: "Analytics".to_string(),
                    icon: rsx!{ Icon { icon: LdPieChart } }
                },
                TechItem {
                    label: "Animations".to_string(),
                    icon: rsx!{ Icon { icon: LdPlay } }
                },
                TechItem {
                    label: "Notifications".to_string(),
                    icon: rsx!{ Icon { icon: LdBell } }
                },
                TechItem {
                    label: "In-app purchase".to_string(),
                    icon: rsx!{ Icon { icon: LdCreditCard } }
                },
            ],
        },
        ExperienceItem {
            journey: vec![
                Position {
                    position: "Freelance software engineer".to_string(),
                    date: "Sep 2021 - Present".to_string(),
                },
            ],
            company: "Toptal".to_string(),
            contract: "Contracter".to_string(),
            points: vec![
                "Built a React Native app prototype for BasedApp (fintech startup) integrating Web3, SafeWallet, Magic Link, and various blockchain functionalities.".to_string(),
                "Developed Quest Social's real-time backend (AdonisJS, MySQL, Socket.io) and React Native mobile app with improved state management, revamped designs and new features, successfully publishing to App Store and Play Store.".to_string(),
                "Prototyped a Flutter mobile app for New York based trading startup.".to_string(),
            ],
            tech: vec![
                TechItem {
                    label: "Flutter".to_string(),
                    icon: rsx!{ Icon { icon: SiFlutter } }
                },
                TechItem {
                    label: "Golang".to_string(),
                    icon: rsx!{ Icon { icon: SiGo } }
                },
                TechItem {
                    label: "React JS".to_string(),
                    icon: rsx!{ Icon { icon: SiReact } }
                },
                TechItem {
                    label: "React Native".to_string(),
                    icon: rsx!{ Icon { icon: SiReact } }
                },
                TechItem {
                    label: "Typescript".to_string(),
                    icon: rsx!{ Icon { icon: SiTypescript } }
                },
                TechItem {
                    label: "Firebase".to_string(),
                    icon: rsx!{ Icon { icon: SiFirebase } }
                },
                TechItem {
                    label: "Node JS".to_string(),
                    icon: rsx!{ Icon { icon: SiNodedotjs } }
                },
                TechItem {
                    label: "Adonis JS".to_string(),
                    icon: rsx!{ Icon { icon: SiAdonisjs } }
                },
                TechItem {
                    label: "Postgres".to_string(),
                    icon: rsx!{ Icon { icon: SiPostgresql } }
                },
                TechItem {
                    label: "Ethereum".to_string(),
                    icon: rsx!{ Icon { icon: SiEthereum } }
                },
                TechItem {
                    label: "Ethers".to_string(),
                    icon: rsx!{ Icon { icon: SiEthers } }
                },
            ],
        },
        ExperienceItem {
            journey: vec![
                Position {
                    position: "Project software engineer".to_string(),
                    date: "Aug 2021 - Oct 2021".to_string(),
                },
            ],
            contract: "Contract".to_string(),
            company: "Voxlabs".to_string(),
            points: vec![
                "Developed a vanilla JavaScript paywall plugin with ad-block bypass capabilities through dynamic CSS implementation, along with a admin panel built using React, TypeScript, Zustand, and Firebase.".to_string(),
                "Maintained an existing Flutter mobile application and did code reviews of junior developers' work.".to_string(),
            ],
            tech: vec![
                TechItem {
                    label: "Flutter".to_string(),
                    icon: rsx!{ Icon { icon: SiFlutter } }
                },
                TechItem {
                    label: "Firebase".to_string(),
                    icon: rsx!{ Icon { icon: SiFirebase } }
                },
                TechItem {
                    label: "Typescript".to_string(),
                    icon: rsx!{ Icon { icon: SiTypescript } }
                },
                TechItem {
                    label: "React JS".to_string(),
                    icon: rsx!{ Icon { icon: SiReact } }
                },
                TechItem {
                    label: "Material UI".to_string(),
                    icon: rsx!{ Icon { icon: SiMaterialdesign } }
                },
            ],
        },
        ExperienceItem {
            journey: vec![
                Position {
                    position: "Software engineer".to_string(),
                    date: "Jan 2021 - Aug 2021".to_string(),
                },
            ],
            company: "Sastaticket.pk".to_string(),
            contract: "Full time".to_string(),
            points: vec![
                "Architected and developed Pakistan's top-ranked airline travel app using Flutter and Bloc design pattern, successfully publishing on App Store and Play Store.".to_string(),
                "Implemented robust development infrastructure including multi-environment configuration and automated workflows for internationalization, boilerplate code, assets generation, and deployment processes.".to_string(),
                "Integrated analytics suite (Webengage, Mixpanel, AppsFlyer, Facebook, Firebase) and multiple payment gateways (Easypaisa, Jazzcash, Paypro, 3D-secured Credit/Debit cards).".to_string(),
                "Hacked a zero cost CI/CD solution using Github Actions and NodeJS for automated APK distribution to Google Drive with email notifications.".to_string(),
            ],
            tech: vec![
                TechItem {
                    label: "Flutter".to_string(),
                    icon: rsx!{ Icon { icon: SiFlutter } }
                },
                TechItem {
                    label: "Bloc".to_string(),
                    icon: rsx!{ Icon { icon: LdFileCode } }
                },
                TechItem {
                    label: "Provider".to_string(),
                    icon: rsx!{ Icon { icon: LdFileCode } }
                },
                TechItem {
                    label: "Firebase".to_string(),
                    icon: rsx!{ Icon { icon: SiFirebase } }
                },
                TechItem {
                    label: "Analytics".to_string(),
                    icon: rsx!{ Icon { icon: LdPieChart } }
                },
                TechItem {
                    label: "Animations".to_string(),
                    icon: rsx!{ Icon { icon: LdPlay } }
                },
                TechItem {
                    label: "Notifications".to_string(),
                    icon: rsx!{ Icon { icon: LdBell } }
                },
                TechItem {
                    label: "Payments".to_string(),
                    icon: rsx!{ Icon { icon: LdCreditCard } }
                },
                TechItem {
                    label: "Google Drive API".to_string(),
                    icon: rsx!{ Icon { icon: SiGoogledrive } }
                },
                TechItem {
                    label: "Node JS".to_string(),
                    icon: rsx!{ Icon { icon: SiNodedotjs } }
                },
                TechItem {
                    label: "Github Actions".to_string(),
                    icon: rsx!{ Icon { icon: LdFileCode } }
                },
            ],
        },
        ExperienceItem {
            journey: vec![
                Position {
                    position: "Freelance software engineer".to_string(),
                    date: "May 2018 - Oct 2021".to_string(),
                },
            ],
            company: "Upwork  / Fiverr".to_string(),
            contract: "Contracter".to_string(),
            points: vec![
                "Provided full stack development services for ReactJS, React Native, Flutter, and NodeJS, delivering prototypes and production ready applications".to_string(),
                "Built and published multiple mobile applications including 'Grow Youth App' using React Native with Redux and WordPress API integration, launching on both App Store and Play Store.".to_string(),
                "Prototypes a background video processing app using React Native threads and NodeJS/Dropbox integration.".to_string(),
                "Developed a full-stack small CMS with ExpressJS and React Native App for management of business information repositories.".to_string(),
                "Built Wheelbees, a comprehensive chat application for drivers featuring social authentication, real-time messaging with voice notes and images, advanced search capabilities (text/speech/image recognition), and push notifications.".to_string(),
            ],
            tech: vec![
                TechItem {
                    label: "React JS".to_string(),
                    icon: rsx!{ Icon { icon: SiReact } }
                },
                TechItem {
                    label: "React Native".to_string(),
                    icon: rsx!{ Icon { icon: SiReact } }
                },
                TechItem {
                    label: "Flutter".to_string(),
                    icon: rsx!{ Icon { icon: SiFlutter } }
                },
                TechItem {
                    label: "Bloc".to_string(),
                    icon: rsx!{ Icon { icon: LdFileCode } }
                },
                TechItem {
                    label: "Redux".to_string(),
                    icon: rsx!{ Icon { icon: SiRedux } }
                },
                TechItem {
                    label: "Redux Saga".to_string(),
                    icon: rsx!{ Icon { icon: SiRedux } }
                },
                TechItem {
                    label: "Node JS".to_string(),
                    icon: rsx!{ Icon { icon: SiNodedotjs } }
                },
                TechItem {
                    label: "Adonis JS".to_string(),
                    icon: rsx!{ Icon { icon: SiAdonisjs } }
                },
                TechItem {
                    label: "Asana API".to_string(),
                    icon: rsx!{ Icon { icon: SiAsana } }
                },
                TechItem {
                    label: "Dropbox API".to_string(),
                    icon: rsx!{ Icon { icon: SiDropbox } }
                },
                TechItem {
                    label: "MYSQL".to_string(),
                    icon: rsx!{ Icon { icon: SiMysql } }
                },
                TechItem {
                    label: "Firebase".to_string(),
                    icon: rsx!{ Icon { icon: SiFirebase } }
                },
                TechItem {
                    label: "Notifications".to_string(),
                    icon: rsx!{ Icon { icon: LdBell } }
                },
                TechItem {
                    label: "AWS (S3, SES, RDS, EC2)".to_string(),
                    icon: rsx!{ Icon { icon: SiAmazon } }
                },
            ],
        },
        ExperienceItem {
            company: "Fetch Sky".to_string(),
            contract: "Full time".to_string(),
            journey: vec![
                Position {
                    position: "Software engineer".to_string(),
                    date: "Dec 2016 - Oct 2018".to_string(),
                },
            ],
            points: vec![
                "Worked on Peekaboo Guru, a location-based and lifestyle platform, developing its NodeJS/MySQL backend, ReactJS web app, and React Native mobile app.".to_string(),
                "Built custom modules including a lightweight image cropping and sizing module using node-canvas, integrated Redux ecosystem (Redux-Saga, ImmutableJS) for state management, and implemented third-party services like Firebase analytics and OneSignal notifications.".to_string(),
                "Developed internal tools including an AngularJS admin panel and a server-side rendered analytics dashboard with PDF reporting capabilities.".to_string(),
            ],
            tech: vec![
                TechItem {
                    label: "React JS".to_string(),
                    icon: rsx!{ Icon { icon: SiReact } }
                },
                TechItem {
                    label: "React Native".to_string(),
                    icon: rsx!{ Icon { icon: SiReact } }
                },
                TechItem {
                    label: "Redux".to_string(),
                    icon: rsx!{ Icon { icon: SiRedux } }
                },
                TechItem {
                    label: "Redux Saga".to_string(),
                    icon: rsx!{ Icon { icon: SiRedux } }
                },
                TechItem {
                    label: "styled-components".to_string(),
                    icon: rsx!{ Icon { icon: SiStyledcomponents } }
                },
                TechItem {
                    label: "webpack".to_string(),
                    icon: rsx!{ Icon { icon: SiWebpack } }
                },
                TechItem {
                    label: "Firebase".to_string(),
                    icon: rsx!{ Icon { icon: SiFirebase } }
                },
                TechItem {
                    label: "Analytics".to_string(),
                    icon: rsx!{ Icon { icon: LdPieChart } }
                },
                TechItem {
                    label: "Node JS".to_string(),
                    icon: rsx!{ Icon { icon: SiNodedotjs } }
                },
                TechItem {
                    label: "MYSQL".to_string(),
                    icon: rsx!{ Icon { icon: SiMysql } }
                },
                TechItem {
                    label: "Angular JS".to_string(),
                    icon: rsx!{ Icon { icon: SiAngular } }
                },
                TechItem {
                    label: "Notifications".to_string(),
                    icon: rsx!{ Icon { icon: LdBell } }
                },
                TechItem {
                    label: "Puppeteer".to_string(),
                    icon: rsx!{ Icon { icon: SiGooglechrome } }
                },
                TechItem {
                    label: "AWS (S3, SES)".to_string(),
                    icon: rsx!{ Icon { icon: SiAmazon } }
                },
            ],
        },
    ]
}

pub fn AppExperience() -> Element {
    let experience = get_experience();

    // Define contract colors mapping
    let contract_colors = HashMap::from([
        (
            "Full time",
            "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300",
        ),
        (
            "Freelance",
            "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300",
        ),
        (
            "Contract",
            "bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-300",
        ),
        (
            "Contracter",
            "bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-300",
        ),
    ]);

    rsx! {
        div { class: "flex bg-zinc-100 dark:bg-zinc-900",
            div { class: "container max-w-8xl mx-auto px-6 py-10",
                h2 { class: "text-2xl font-semibold text-zinc-800 dark:text-zinc-100 mb-8",
                    "Experience"
                }

                div { class: "flex flex-col gap-8",
                    {experience.iter().enumerate().map(|(index, exp)| {
                        let contract_color = contract_colors.get(exp.contract.as_str())
                            .unwrap_or(&"bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-300");

                        rsx! {
                            div { key: "{index}",
                                // Company Name
                                div { class: "flex flex-row items-center gap-3",
                                    h3 { class: "text-xl font-semibold text-zinc-800 dark:text-zinc-100",
                                        "{exp.company}"
                                    }
                                    p { class: "text-xs px-2 py-0.5 rounded-full {contract_color}",
                                        "{exp.contract}"
                                    }
                                }

                                // Positions & Dates
                                div { class: "mt-2 mb-4 space-y-1",
                                    {exp.journey.iter().enumerate().map(|(idx, position)| {
                                        rsx! {
                                            div { key: "{idx}", class: "flex flex-col",
                                                span { class: "text-sm font-medium text-zinc-700 dark:text-zinc-300",
                                                    "{position.position}"
                                                }
                                                span { class: "text-sm text-zinc-600 dark:text-zinc-400",
                                                    "{position.date}"
                                                }
                                            }
                                        }
                                    })}
                                }

                                // Tech Stack
                                div { class: "flex flex-wrap sm:gap-3 gap-1.5 mb-6",
                                    {exp.tech.iter().enumerate().map(|(idx, tech)| {
                                        rsx! {
                                            div { key: "{idx}",
                                                class: "flex items-center cursor-pointer
                                                sm:gap-2 gap-1.5 text-xs
                                                sm:px-3 sm:py-1 py-0.5 px-2.5
                                                bg-zinc-200 dark:bg-zinc-800
                                                text-zinc-800 dark:text-zinc-200
                                                rounded-full font-medium font-mono
                                                hover:bg-zinc-300 dark:hover:bg-zinc-700
                                                transition-colors duration-200",
                                                div { class: "sm:w-3.5 w-3",
                                                    {tech.icon.clone()}
                                                }
                                                "{tech.label}"
                                            }
                                        }
                                    })}
                                }

                                // Responsibilities
                                ul { class: "list-disc list-outside ml-4 space-y-2 text-zinc-700 dark:text-zinc-300 marker:text-zinc-500",
                                    {exp.points.iter().enumerate().map(|(idx, point)| {
                                        rsx! {
                                            li { key: "{idx}", class: "pl-2",
                                                "{point}"
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
