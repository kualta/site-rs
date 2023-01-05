#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::*;

pub const PUBLIC_URL: &str = "/";
pub const LECTRO_GRADIENT: &str =
    "inline-block bg-gradient-to-r from-red-200 via-red-300 to-yellow-200 text-transparent bg-clip-text";
pub const RUST_GRADIENT: &str =
    "inline-block bg-gradient-to-r from-red-300 via-red-200  to-yellow-100 text-transparent bg-clip-text";
pub const CPP_GRADIENT: &str =
    "inline-block bg-gradient-to-r from-sky-300 to-sky-200 text-transparent bg-clip-text";
pub const TS_GRADIENT: &str =
    "inline-block bg-gradient-to-r from-violet-300 to-violet-200 text-transparent bg-clip-text";

fn main() {
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
            div { class: "flex flex-row place-items-center place-content-around w-full",
                links
            }
        }
    })
}

fn Home(cx: Scope) -> Element {
    let languages = rsx! {
        span { class: "flex flex-row",
            p { ">  " }
            p { class: RUST_GRADIENT, "Rust" }
            p { ", C++, TypeScript" }
        }
    };

    cx.render(rsx! {
        TopBar { }
        div { class: "flex flex-col items-center place-content-evenly lg:flex-row h-full",
            div { class: "flex flex-col items-left place-content-around h-24",
                h3 { " > Software engineer" }
                languages
                h3 { " > I like to keep things clean" }
            }
            div { class: "flex flex-col text-right place-content-around h-16",
                h3 { "github: " a { class: "underline", href:"https://github.com/lectromoe", "@lectromoe" } }
                h3 { " " }
                h3 { "hr: " a { class: "underline", href:"mailto:jobs@lectro.moe", "jobs@lectro.moe" }  }
        
            }
        }
    })
}

fn Projects(cx: Scope) -> Element {
    cx.render(rsx! {
        TopBar { }
        " proj "
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
