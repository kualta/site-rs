#![allow(non_snake_case)]

mod content;

use content::{Article, Contact, Project};
use dioxus::prelude::*;
use dioxus_router::*;
use serde::de::DeserializeOwned;

pub const PUBLIC_URL: &str = "/";
pub const LECTRO_GRADIENT: &str =
    "inline-block bg-gradient-to-r from-red-200 via-red-300 to-yellow-200 text-transparent bg-clip-text";
pub const RUST_GRADIENT: &str =
    "inline-block bg-gradient-to-r from-red-300 via-red-200  to-yellow-100 text-transparent bg-clip-text";

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Background { }
        main { class: "roboto-mono flex flex-col w-max h-screen sm:w-3/4 xl:w-1/2 mx-auto text-xl text-stone-200",
            Router {
                Route { to: "/", Home {} }
                Route { to: "/projects", Projects {} }
                Route { to: "/articles", Articles {} }
                Route { to: "/contacts", Contacts {} }
            }
        }
    })
}

fn TopBar(cx: Scope) -> Element {
    let title_class = use_state(cx, || {
        format!(
            "p-8 text-transparent bg-clip-text text-3xl font-bold {}",
            LECTRO_GRADIENT
        )
    });
    let title = rsx! { h3 { class: title_class.as_str(), "lectro.moe"}};

    let router = use_route(cx);
    let url = router.last_segment()?;
    let links = match url {
        "" => rsx! {
            Link { to: "/projects", "projects" }
            Link { to: "/articles", "articles" }
            Link { to: "/contacts", "contacts" }
        },
        _ => rsx! {
            Link { to: "/", "< back" }
        },
    };

    cx.render(rsx! {
        div {
            class: "flex flex-col items-center",
            Link { to: "/", title }
            div { class: "flex flex-row text-lg place-items-center place-content-around w-full",
                links
            }
        }
    })
}

fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        TopBar { }
        div { class: "flex flex-col items-center place-content-evenly lg:flex-row h-full",
            div { class: "flex flex-col items-left place-content-around h-24",
                h3 { " > Software engineer" }
                span { class: "flex flex-row",
                    p { ">  " }
                    p { class: RUST_GRADIENT, "Rust" }
                    p { ", C++, TypeScript" }
                }
                h3 { " > I like to keep things clean" }
            }
            div { class: "flex flex-col text-right place-content-around h-16",
                h3 { "github: " a { class: "", href:"https://github.com/lectromoe", "@lectromoe" } }
                h3 {  "hr: "  a { class: RUST_GRADIENT, href: "mailto:jobs@lectro.moe", "jobs@lectro.moe" } }
            }
        }
    })
}

async fn fetch_data<T>(url: &str) -> Result<T, reqwest::Error>
where
    T: DeserializeOwned,
{
    let data: T = reqwest::get(url).await?.json::<T>().await?;
    Ok(data)
}

fn Projects(cx: Scope) -> Element {
    let url = "https://raw.githubusercontent.com/lectromoe/Data/master/projects.json";

    let projects = use_future(cx, (), |_| async move {
        match fetch_data::<Vec<Project>>(url).await {
            Ok(projects) => projects,
            Err(err) => {
                log::error!("{:?}", err);
                vec![]
            }
        }
    });

    let response = format!("{:?}", projects.value());

    cx.render(rsx! {
        TopBar { }
        match projects.value() {
            Some(project) => rsx! {
                table { class: "text-base",
                    project.iter().map(|project| rsx! {
                        tr { class: "border-gray-700 border-y-2 font-medium text-left",
                            td { class: "text-center", project.status.as_str() }
                            td { class: "font-bold text-center", project.name.as_str() }
                            td { class: "", project.description.as_str() }
                            td { class: "", project.language.as_str() }
                            td { class: "", project.stack.as_str() }
                            td { class: "", project.links.iter().map(|link| rsx! {
                                a { class: "underline", href: link.1.as_str(), link.0.as_str()}})
                            }
                        }
                    })
                }
            },
            None => rsx! {
                div { class: "mx-auto",
                    div {
                        p { "Loading projects..." }
                    }
                }
            }
        }
    })
}

fn Articles(cx: Scope) -> Element {
    cx.render(rsx! {
        TopBar { }
        " article "
    })
}

fn Contacts(cx: Scope) -> Element {
    cx.render(rsx! {
        TopBar { }
        "contacts"
    })
}

fn Background(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "absolute inset-0 -z-10 h-full w-full bg-[#101112]"
        }
    })
}
