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
        main {
            class: "roboto-mono flex flex-col w-full md:w-3/4 xl:w-1/2 mx-auto text-stone-200",
            Router {
                Route { to: "/", Home {} }
                Route { to: "/projects", Projects {} }
                Route { to: "/articles", Articles {} }
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

    let bar = match url {
        "" => rsx! {
                div {
                    class: "flex w-full flex-row place-items-center place-content-around w-full",
                    Link { to: "/projects", "projects" }
                    Link { to: "/articles", "articles" }
                    Link { to: "/contacts", "contacts" }
                }
        },
        _ => rsx! {
                div {
                    class: "flex w-full flex-row place-items-center place-content-around w-full",
                    Link { to: "/", "< back" }
                }
        },
    };

    cx.render(rsx! {
        div {
            class: "flex flex-col items-center",
            Link { to: "/", title }
            bar
        }
    })
}

fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            TopBar { }
            { " hi "}
        }
    })
}

fn Projects(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            TopBar { }
            { " proj "}
        }
    })
}

fn Articles(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            TopBar { }
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
            class: "absolute inset-0 -z-10 h-full w-full bg-[#101112]"
        }
    })
}
