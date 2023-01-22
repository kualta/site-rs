#![allow(non_snake_case)]

mod content;

use content::{Article, Contact, Data, Project};
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::{
    BsCaretDown, BsDiscord, BsEnvelopeFill, BsGithub, BsLink45deg, BsMastodon, BsTelegram,
    BsTwitter,
};
use dioxus_free_icons::Icon;
use dioxus_router::*;
use serde::de::DeserializeOwned;

pub const PUBLIC_URL: &str = "/";
pub const DATA_URL: &str = "https://raw.githubusercontent.com/lectromoe/Data/master/data.json";

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let data = use_future(cx, (), |_| async move {
        match fetch_data::<Data>(DATA_URL).await {
            Ok(data) => data,
            Err(err) => {
                log::error!("{:?}", err);
                panic!()
            }
        }
    });

    let (projects, articles, contacts) = if let Some(data) = data.value() {
        (
            data.projects.clone(),
            data.articles.clone(),
            data.contacts.clone(),
        )
    } else {
        (vec![], vec![], vec![])
    };

    cx.render(rsx! {
        main { class: "flex flex-col w-screen h-max lg:w-5/6 xl:w-2/3 mx-auto text-xl text-stone-200",
            Router {
                TopBar { contacts: contacts.clone() }
                Route { to: "/", HomePage { projects: projects.clone(), articles: articles.clone(), contacts: contacts.clone() } }
                Route { to: "/content", ContentPage {projects: projects, articles: articles} }
                Route { to: "/contacts", ContactsPage {contacts: contacts} }
            }
        }
    })
}

#[inline_props]
fn TopBar(cx: Scope, contacts: Vec<Contact>) -> Element {
    let title_class = use_state(cx, || {
        format!(
            "p-8 text-transparent bg-clip-text text-3xl font-bold {}",
            "inline-block bg-gradient-to-r from-red-200 via-red-300 to-yellow-200 text-transparent bg-clip-text"
        )
    });
    let title = rsx! { h3 { class: title_class.as_str(), "lectro.moe"}};

    let router = use_route(cx);
    let url = router.last_segment()?;
    let links = match url {
        "" => rsx! {
            Link { class: "basis-1/3 text-left", to: "/", title }
            Link { class: "basis-1/3", to: "/", "" }
            Contacts { contacts: contacts.to_vec() }
        },
        _ => rsx! {
            Link { class: "basis-1/3 text-left", to: "/", title }
            Link { class: "basis-1/3", to: "/", "" }
            Contacts { contacts: contacts.to_vec() }
        },
    };

    cx.render(rsx! {
        div { class: "flex flex-row text-xl place-items-baseline place-content-center text-center w-full",
            links
        }
    })
}

#[inline_props]
fn HomePage(
    cx: Scope,
    projects: Vec<Project>,
    articles: Vec<Article>,
    contacts: Vec<Contact>,
) -> Element {
    let gradient = "inline-block bg-gradient-to-r from-red-300 via-red-200  to-yellow-100 text-transparent bg-clip-text";
    cx.render(rsx! {
        div { class: "flex flex-col h-screen grow-0 place-items-between",
            div { class: "roboto-mono flex flex-col items-center place-content-evenly lg:flex-row h-4/5",
                div { class: "flex flex-col items-left place-content-around h-24",
                    h3 { " > Software engineer" }
                    span { class: "flex flex-row",
                        p { ">  " }
                        p { class: gradient, "Rust" }
                        p { ", C++, TypeScript" }
                    }
                    h3 { " > I like to keep things clean" }
                }
                div { class: "flex flex-col text-right place-content-around h-16",
                    h3 { "github: " a { class: "", href:"https://github.com/lectromoe", "@lectromoe" } }
                    h3 { "hr: "  a { class: gradient, href: "mailto:jobs@lectro.moe", "jobs@lectro.moe" } }
                }
            }
            div { class: "h-1/5 flex flex-col justify-beginning place-items-center",
                Link { class: "text-center", to: "/content", 
                    h3 { class: "roboto-mono", "projects" }
                    Icon { class: "mx-auto", icon: BsCaretDown } 
                }
            }
        }
        div { class: "h-screen",
            ContentPage {
                projects: projects.to_vec(),
                articles: articles.to_vec(),
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

#[inline_props]
fn ContentPage(cx: Scope, projects: Vec<Project>, articles: Vec<Article>) -> Element {
    let projects = rsx! {
        table { class: "relative table-auto w-full",
            tr { class: "text-left",
                th { class: "px-4 w-24 sm:w-48", "name" }
                th { class: "",                  "description" }
                th { class: "text-center",       "language" }
                th { class: "",                  "stack" }
                th { class: "",                  "links" }
            }
            projects.iter().map(|project| rsx! {
                tr { class: "border-neutral-700 border-y-2 text-left",
                    td { class: "font-bold px-4",        project.name.as_str() }
                    td { class: "",                      project.description.as_str() }
                    td { class: "text-center",           project.language.as_str() }
                    td { class: "py-2",                  project.stack.as_str() }
                    td { class: "", project.links.iter().map(|link| rsx! {
                        a { class: "underline pr-2", href: link.1.as_str(), link.0.as_str()}})
                    }
                }
            })
        }
    };
    let articles = rsx! {
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
                    td { class: "py-2 text-center",                  article.theme.as_str() }
                    td { class: "", article.links.iter().map(|link| rsx! {
                        a { class: "underline pr-2", href: link.1.as_str(), link.0.as_str()}})
                    }
                }
            })
        }
    };

    cx.render(rsx! {
        div { class: "flex roboto-mono text-center flex-col place-items-center text-xs sm:text-sm md:text-base",
            div { class: "w-full mt-16 mb-10",
                h1 { class: "font-bold text-4xl sm:text-6xl text-neutral-800 my-4", "Projects" }
                projects
            }
            div { class: "w-full mt-16 mb-10",
                h1 { class: "font-bold text-4xl sm:text-6xl text-neutral-800 my-4", "Articles" }
                articles
            }
        }
    })
}

#[rustfmt::skip]
fn SocialsIcon(name: &str) -> LazyNodes<'_, '_>{
    match name {
        "github"   => rsx! { Icon { width: 25, height: 25, icon: BsGithub } },
        "mail"     => rsx! { Icon { width: 25, height: 25, icon: BsEnvelopeFill } },
        "telegram" => rsx! { Icon { width: 25, height: 25, icon: BsTelegram } },
        "mastodon" => rsx! { Icon { width: 25, height: 25, icon: BsMastodon } },
        "discord"  => rsx! { Icon { width: 25, height: 25, icon: BsDiscord } },
        "twitter"  => rsx! { Icon { width: 25, height: 25, icon: BsTwitter } },
        _          => rsx! { Icon { width: 25, height: 25, icon: BsLink45deg } },
    }
}

#[inline_props]
fn Contacts(cx: Scope, contacts: Vec<Contact>) -> Element {
    let contacts = contacts.iter().map(|contact| {
        let icon = SocialsIcon(&contact.name);

        rsx! {
            a { class: "",
                href: contact.link.as_str(),
                icon
            }
        }
    });

    cx.render(rsx! {
        div { class: "flex flex-row place-items-center justify-center space-x-4",
            contacts
        }
    })
}

#[inline_props]
fn ContactsPage(cx: Scope, contacts: Vec<Contact>) -> Element {
    cx.render(rsx! {
        div { class: "mx-auto my-auto",
            div { class: "flex flex-row min-w-fit space-x-4",
                Contacts { contacts: contacts.to_vec() }
            }
        }
    })
}
