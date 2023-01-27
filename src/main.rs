#![allow(non_snake_case)]

mod content;

use std::ops::Deref;

use content::{Article, Contact, Data, Project};
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::{
    BsCaretDown, BsDiscord, BsEnvelopeFill, BsGithub, BsLink45deg, BsMastodon, BsTelegram,
    BsTwitter, BsYoutube,
};
use dioxus_free_icons::Icon;
use dioxus_router::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
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
        main { class: "mx-auto flex flex-col min-h-screen max-w-2xl text-stone-200",
            Header { contacts: contacts }
            Projects { projects: projects }
            Articles { articles: articles }
        }
    })
}

#[inline_props]
fn Header(cx: Scope, contacts: Vec<Contact>) -> Element {
    let mut rng = rand::thread_rng();
    let dict = vec![
        "simply makes things", 
        "makes simple things",
        "makes things simple", 
    ];
    let text = dict.choose(&mut rng).unwrap().deref();
    cx.render(rsx! {
        header { class: "container",
            div { class: "flex items-center justify-between roboto-mono border-neutral-800 border-b py-4",
                span { b { "kualta "} text }
                Contacts { contacts: contacts.to_vec() }
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
fn Projects(cx: Scope, projects: Vec<Project>) -> Element {
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
                    td { class: "px-4 font-bold",             project.name.as_str() }
                    td { class: "px-4",                       project.description.as_str() }
                    td { class: "px-4 text-center",           project.language.as_str() }
                    td { class: "px-4 py-2",                  project.stack.as_str() }
                    td { class: "px-4", project.links.iter().map(|link| rsx! {
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
        }
    })
}

#[inline_props]
fn Articles(cx: Scope, articles: Vec<Article>) -> Element {
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
            div { class: "mt-16 mb-10",
                h1 { class: "font-bold text-4xl sm:text-6xl text-neutral-800 my-4", "Articles" }
                articles
            }
        }
    })
}

#[rustfmt::skip]
fn SocialIcon(name: &str) -> LazyNodes<'_, '_>{
    match name {
        "github"   => rsx! { Icon { width: 20, height: 20, icon: BsGithub } },
        "mail"     => rsx! { Icon { width: 20, height: 20, icon: BsEnvelopeFill } },
        "telegram" => rsx! { Icon { width: 20, height: 20, icon: BsTelegram } },
        "mastodon" => rsx! { Icon { width: 20, height: 20, icon: BsMastodon } },
        "discord"  => rsx! { Icon { width: 20, height: 20, icon: BsDiscord } },
        "twitter"  => rsx! { Icon { width: 20, height: 20, icon: BsTwitter } },
        "youtube"  => rsx! { Icon { width: 20, height: 20, icon: BsYoutube } },
        _          => rsx! { Icon { width: 20, height: 20, icon: BsLink45deg } },
    }
}

#[inline_props]
fn Contacts(cx: Scope, contacts: Vec<Contact>) -> Element {
    let contacts = contacts.iter().map(|contact| {
        let icon = SocialIcon(&contact.name);

        rsx! {
            a { href: contact.link.as_str(), icon }
        }
    });

    cx.render(rsx! {
        div { class: "flex flex-row place-items-center justify-center space-x-4",
            contacts
        }
    })
}
