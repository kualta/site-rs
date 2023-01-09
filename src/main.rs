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
        main { class: "flex flex-col w-screen h-screen lg:w-5/6 xl:w-2/3 mx-auto text-xl text-stone-200",
            Router {
                Route { to: "/", Home {} }
                Route { to: "/content", Content {} }
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
            Link { class: "basis-1/3", to: "/content", "content" }
            Link { class: "basis-1/3", to: "/", title }
            Link { class: "basis-1/3", to: "/contacts", "contacts" }
        },
        "content" => rsx! {
            Link { class: "basis-1/3", to: "/", "< back" }
            Link { class: "basis-1/3", to: "/", title }
            Link { class: "basis-1/3", to: "/contacts", "contacts" }
        },
        "contacts" => rsx! {
            Link { class: "basis-1/3", to: "/", "< back" }
            Link { class: "basis-1/3", to: "/", title }
            Link { class: "basis-1/3", to: "/content", "content" }
        },
        _ => rsx! {
            Link { class: "basis-1/3", to: "/", "< back" }
            Link { class: "basis-1/3", to: "/", title }
            Link { class: "basis-1/3", to: "/contacts", "contacts" }
        },
    };

    cx.render(rsx! {
        div { class: "flex flex-row text-xl place-items-baseline place-content-center text-center w-full",
            links
        }
    })
}

fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        TopBar { }
        div { class: "roboto-mono flex flex-col items-center place-content-evenly lg:flex-row h-full",
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

fn Content(cx: Scope) -> Element {
    let projects_url = "https://raw.githubusercontent.com/lectromoe/Data/master/projects.json";
    let articles_url = "https://raw.githubusercontent.com/lectromoe/Data/master/articles.json";

    let projects = use_future(cx, (), |_| async move {
        match fetch_data::<Vec<Project>>(projects_url).await {
            Ok(projects) => projects,
            Err(err) => {
                log::error!("{:?}", err);
                vec![]
            }
        }
    });
    let articles = use_future(cx, (), |_| async move {
        match fetch_data::<Vec<Article>>(articles_url).await {
            Ok(articles) => articles,
            Err(err) => {
                log::error!("{:?}", err);
                vec![]
            }
        }
    });

    let projects = rsx! {
        match projects.value() {
            Some(project) => rsx! {
                table { class: "relative table-auto w-full",
                    tr { class: "text-left",
                        th { class: "px-4 w-24 sm:w-48",    "name" }
                        th { class: "",             "description" }
                        th { class: "text-center",  "language" }
                        th { class: "",             "stack" }
                        th { class: "",   "links" }
                    }
                    project.iter().map(|project| rsx! {
                        tr { class: "border-neutral-700 border-y-2 text-left",
                            td { class: "font-bold px-4", project.name.as_str() }
                            td { class: "",                      project.description.as_str() }
                            td { class: "text-center",           project.language.as_str() }
                            td { class: "py-4",                  project.stack.as_str() }
                            td { class: "", project.links.iter().map(|link| rsx! {
                                a { class: "underline pr-2", href: link.1.as_str(), link.0.as_str()}})
                            }
                        }
                    })
                }
            },
            None => rsx! {
                div { class: "mx-auto",
                    div {
                        p { "No projects... Too bad!" }
                    }
                }
            }
        }
    };
    let articles = rsx! {
        match articles.value() {
            Some(articles) => rsx! {
                table { class: "relative table-auto w-full",
                    tr { class: "text-left",
                        th { class: "px-4 w-24 sm:w-48", "name" }
                        th { class: "",            "description" }
                        th { class: "text-center", "theme" }
                        th { class: "",            "links" }
                    }
                    articles.iter().map(|article| rsx! {
                        tr { class: "border-neutral-700 border-y-2 text-left",
                            td { class: "font-bold px-4", article.name.as_str() }
                            td { class: "",                      article.description.as_str() }
                            td { class: "py-4 text-center",                  article.theme.as_str() }
                            td { class: "", article.links.iter().map(|link| rsx! {
                                a { class: "underline pr-2", href: link.1.as_str(), link.0.as_str()}})
                            }
                        }
                    })
                }
            },
            None => rsx! {
                div { class: "mx-auto",
                    div {
                        p { "No articles... Something went wrong!" }
                    }
                }
            }
        }
    };

    cx.render(rsx! {
        TopBar { }
        div { class: "flex flex-col place-items-center text-xs sm:text-sm md:text-base",
            div { class: "roboto-mono text-left w-full mt-4",
                h1 { class: "font-bold text-4xl sm:text-6xl text-neutral-800 my-4", "Projects" }
                projects
            }
            div { class: "roboto-mono text-right w-full mt-16",
                h1 { class: "font-bold text-4xl sm:text-6xl text-neutral-800 my-4", "Articles" }
                articles
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
