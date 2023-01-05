#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::*;

pub const PUBLIC_URL: &str = "/";
pub const LECTRO_GRADIENT: &str = "bg-gradient-to-r from-red-200 via-red-300 to-yellow-200";

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Background { }
        Avatar { }
        Router {
            Route { to: "/", Home {} }
            Route { to: "/projects", Projects {} }
            Route { to: "/articles", Articles {} }
        }
    })
}

fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            { " hi "}
        }
    })
}

fn Projects(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            { " proj "}
        }
    })
}

fn Articles(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            { " article "}
        }
    })
}

fn Avatar(cx: Scope) -> Element {
    let onclick = |_| println!("click!");
    let animation = use_state(cx, || "");
    let class = use_state(cx, || {
        format!(
            "flex shrink-0 items-center rounded-full p-1 m-4 w-20 {} {}",
            LECTRO_GRADIENT, *animation
        )
    });

    cx.render(rsx! {
        div {
            class: class.as_str(),
            role: "button", class: "", onclick: onclick,
            img { src: "assets/img/ps2doggy.png", alt: "doggy", class: "rounded-full",
            }
        }
    })
}

fn Background(cx: Scope) -> Element {
    cx.render(rsx! {
        div {

        }
    })
}
