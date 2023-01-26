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
pub const DATA_URL: &str = "https://raw.githubusercontent.com/kualta/Data/master/data.json";

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
                Route { to: "/", 
                    HomePage { contacts: contacts } 
                    ContentPage { projects: projects, articles: articles }
                }
            }
        }
    })
}

#[inline_props]
fn HomePage(cx: Scope, contacts: Vec<Contact>) -> Element {
    let gradient = "inline-block bg-gradient-to-r from-red-300 via-red-200 to-yellow-100 text-transparent bg-clip-text";
    let title = rsx! { h3 { class: "text-transparent bg-clip-text text-3xl font-bold inline-block bg-gradient-to-r  
        from-red-200 via-red-300 to-yellow-200 text-transparent bg-clip-text", "kualta"}};

    cx.render(rsx! {
        div { class: "flex flex-col h-screen grow-0 justify-center place-items-center space-y-8",
            div {
                title
            }
            div { class: "roboto-mono flex flex-col items-center place-content-evenly space-x-12 lg:flex-row",
                div { class: "flex flex-col items-left place-content-around h-24",
                    h3 { " > Software engineer" }
                    span { class: "flex flex-row",
                        p { ">  " }
                        p { class: gradient, "Rust" }
                        p { ", C++, TypeScript" }
                    }
                    h3 { " > I like to keep things clean" }
                }
                div { class: "flex flex-col text-right justify-center h-24",
                    h3 { "github: " a { class: "", href:"https://github.com/kualta", "@kualta" } }
                    h3 { "hr: "  a { class: gradient, href: "mailto:jobs@kualta.dev", "jobs@kualta.dev" } }
                }
            }
            div { class: "",
                Contacts { contacts: contacts.to_vec() }
            }
        }
        div { class: "absolute bottom-0 inset-x-0 p-8",
            a { class: "text-center", href: "#content", 
                h3 { class: "roboto-mono", "projects" }
                Icon { class: "mx-auto", icon: BsCaretDown } 
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
        table { class: "relative table-auto",
            tr { class: "text-left",
                th { class: "px-4 ", "name" }
                th { class: "px-4 ",                  "description" }
                th { class: "px-4 text-center",       "language" }
                th { class: "px-4 ",                  "stack" }
                th { class: "px-4 ",                  "links" }
            }
            projects.iter().map(|project| rsx! {
                tr { class: "border-neutral-800 border-y-2 text-left",
                    td { class: "px-4 font-bold",        project.name.as_str() }
                    td { class: "px-4",                      project.description.as_str() }
                    td { class: "px-4 text-center",           project.language.as_str() }
                    td { class: "px-4 py-2",                  project.stack.as_str() }
                    td { class: "px-4", project.links.iter().map(|link| rsx! {
                        a { class: "underline pr-2", href: link.1.as_str(), link.0.as_str()}})
                    }
                }
            })
        }
    };
    let articles = rsx! {
        table { class: "relative table-auto",
            tr { class: "text-left",
                th { class: "px-4", "name" }
                th { class: "px-4",            "description" }
                th { class: "px-4 text-center", "theme" }
                th { class: "px-4",            "links" }
            }
            articles.iter().map(|article| rsx! {
                tr { class: "border-neutral-800 border-y-2 text-left",
                    td { class: "px-4 font-bold", article.name.as_str() }
                    td { class: "px-4",                      article.description.as_str() }
                    td { class: "px-4 py-2 text-center",                  article.theme.as_str() }
                    td { class: "px-4", article.links.iter().map(|link| rsx! {
                        a { class: "underline pr-2", href: link.1.as_str(), link.0.as_str()}})
                    }
                }
            })
        }
    };

    cx.render(rsx! {
        div { class: "h-screen flex roboto-mono text-center flex-col place-items-center justify-center text-xs sm:text-sm md:text-base", id: "content",
            div { class: "mt-16 mb-10 ",
                h1 { class: "font-bold text-4xl sm:text-6xl text-neutral-800 my-4", "Projects" }
                projects
            }
            div { class: "mt-16 mb-10",
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
