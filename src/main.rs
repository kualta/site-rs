#![allow(non_snake_case)]

mod content;

use content::{DataSchema, Entry};
use dioxus::html::title;
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::{
    BsDiscord, BsEnvelopeFill, BsGithub, BsLink45deg, BsMastodon, BsTelegram, BsTwitter, BsYoutube,
};
use dioxus_free_icons::Icon;
use rand::seq::SliceRandom;
use serde::de::DeserializeOwned;
use std::ops::Deref;

pub const PUBLIC_URL: &str = "/";
pub const DATA_URL: &str = "https://raw.githubusercontent.com/kualta/Data/master/data.json";

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let data = use_future(cx, (), |_| async move {
        match fetch_data::<DataSchema>(DATA_URL).await {
            Ok(data) => data,
            Err(err) => {
                log::error!("{:?}", err);
                panic!()
            }
        }
    });

    let (projects, posts, contacts) = if let Some(data) = data.value() {
        (
            data.projects.clone(),
            data.articles.clone(),
            data.contacts.clone(),
        )
    } else {
        (vec![], vec![], vec![])
    };

    cx.render(rsx! {
        main { class: "mx-auto flex flex-col min-h-screen max-w-2xl text-stone-200 px-4",
            Header { contacts: contacts.clone() }
            div { class: "flex-1",
                div { class: "container flex flex-col",
                    DataList { data: projects, list_title: "Projects" }
                    DataList { data: posts, list_title: "Articles" }
                    DataList { data: contacts, list_title: "Contacts" }
                }
            }
        }
    })
}

#[inline_props]
fn Header(cx: Scope, contacts: Vec<Entry>) -> Element {
    let mut rng = rand::thread_rng();
    let dict = vec![
        "simply makes things",
        "makes simple things",
        "makes things simple",
    ];
    let text = dict.choose(&mut rng).unwrap().deref(); 
    // TODO: change text every n seconds

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
fn DataList<'a>(cx: Scope, data: Vec<Entry>, list_title: &'a str) -> Element {
    cx.render(rsx! {
        div {class: "container", 
            h2 { class: "font-semibold text-xl roboto-mono pt-14", list_title.deref() }
            ul { class: "grid gap-2 py-6 border-neutral-800 border-b list-disc list-inside",
                data.iter().map(|project| rsx! {
                    li {
                        a { class: "py-1 roboto-mono",
                            href: project.link.as_str(),
                            span { class: "font-bold", project.name.as_str() }
                            span { " " }
                            span { class: "underline decoration-1 underline-offset-2 decoration-neutral-400", project.description.as_str() }
                            match &project.theme {
                                Some(theme) => rsx! { span { " (" theme.as_str()  ") "} },
                                None => rsx!("")
                            }
                        }
                    }
                })
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
fn Contacts(cx: Scope, contacts: Vec<Entry>) -> Element {
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
