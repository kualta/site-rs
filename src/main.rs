#![allow(clippy::let_unit_value)]

#[rustfmt::skip]
mod styles;
mod project;

use gloo::timers::callback::Timeout;
use project::*;
use rand::prelude::*;
use styles::*;
use web_sys::HtmlAudioElement;
use yew::prelude::*;

#[function_component(Avatar)]
pub fn avatar() -> Html {
    let animation = use_state(|| "");
    let is_active = use_state(|| true);

    if !*is_active {
        let active = is_active.clone();
        Timeout::new(1000, move || {
            active.set(true);
        })
        .forget();
    }
    let onclick = {
        let result = rand::thread_rng().gen_range(0..15);
        let animation = animation.clone();

        if !*is_active {
            Callback::from(|_| {})
        } else if result == 0 {
            Callback::from(move |_| {
                let _ = HtmlAudioElement::new_with_src("/assets/sound/medal_click_rare.wav")
                    .expect("Failed to load resource")
                    .play();
                is_active.set(false);
                animation.set("flip");
            })
        } else {
            Callback::from(move |_| {
                let _ = HtmlAudioElement::new_with_src("/assets/sound/medal_click.wav")
                    .expect("Failed to load resource")
                    .play();
                if *animation == "shake-x" {
                    animation.set("shake-y");
                } else {
                    animation.set("shake-x");
                }
            })
        }
    };

    html! {
        <div role="button" class={format!("flex shrink-0 items-center rounded-full p-1 m-4 w-20 {} {}", LECTRO_GRADIENT, *animation)} {onclick}>
            <img src="assets/img/ps2doggy.png" alt="doggy" class="rounded-full"/>
        </div>
    }
}

#[function_component(TopBar)]
pub fn top_bar() -> Html {
    html! {
        <div class="flex flex-col lg:flex-row items-center place-content-start p-4">
            <Avatar />
            <h3 class={format!("p-8 text-transparent bg-clip-text text-3xl font-bold {}", LECTRO_GRADIENT)}>
                { "lectro.moe" }
            </h3>
            <div class="flex flex-row lg:ml-auto text-xl font-medium p-8 gap-16">
                <a href="#projects" class="transition-all hover:text-fuchsia-300"> { "projects" } </a>
                <a href="#articles" class="transition-all hover:text-indigo-200">  { "articles" } </a>
                <a href="#contacts" class="transition-all hover:text-green-200">   { "contacts" } </a>
            </div>
        </div>
    }
}

#[function_component(Status)]
pub fn status() -> Html {
    html! {
        <div id="status" class="roboto-mono mono flex flex-grow flex-col text-3xl text-left place-content-center gap-8 p-4">
            <div>
                { "> I write Games, Engines and Tools " }
            </div>
            <div>
                { "> Mostly in " }
                <a href="https://www.rust-lang.org/" class={format!("inline-block {}", RUST_GRADIENT)}>{ "Rust" }</a>
            </div>
            <div>
                { "> " }
                <a href="#contacts" class={format!("inline-block {}", CONTACTS_GRADIENT)}>{ " Open " }</a>
                <a href="#contacts" class={format!("", )}>{ " for job proposals" }</a>
            </div>
        </div>
    }
}

#[function_component(Projects)]
pub fn projects() -> Html {
    let projects = vec![
        Project {
            name: "Fuji Bot".to_owned(),
            description: "a hardcore hypercasual game".to_owned(),
            date: "SEP 2019".to_owned(),
            status: StatusTag::Complete,
            lang: LangTag::Cs,
            stack: vec![StackTag::Unity],
            links: vec![
                ProjectLink::GitHub("https://github.com/lectromoe/fuji-bot".to_owned()),
                ProjectLink::GooglePlay(
                    "https://play.google.com/store/apps/details?id=com.lectroMathew.FujiBot&hl=en&gl=US".to_owned(),
                ),
            ],
        },
        Project {
            name: "Core Engine".to_owned(),
            description: "a neat game engine".to_owned(),
            date: "JAN 2021".to_owned(),
            status: StatusTag::Archived,
            lang: LangTag::Cpp,
            stack: vec![StackTag::Core],
            links: vec![ProjectLink::GitHub(
                "https://github.com/lectromoe/Core".to_owned(),
            )],
        },
        Project {
            name: "Hemi Typer".to_owned(),
            description: "an experimental typing tutor".to_owned(),
            date: "MAR 2022".to_owned(),
            status: StatusTag::Complete,
            lang: LangTag::Rust,
            stack: vec![StackTag::Wasm, StackTag::Dioxus],
            links: vec![
                ProjectLink::GitHub("https://github.com/lectromoe/HemiTyper".to_owned()),
                ProjectLink::Mirror("https://lectromoe.github.io/HemiTyper/".to_owned()),

                // TODO: Update when hosted on lectro.moe
                ProjectLink::Use("https://lectromoe.github.io/HemiTyper/".to_owned()), 
            ],
        },
        Project {
            name: "Pomodoro".to_owned(),
            description: "a simple pomo timer".to_owned(),
            date: "AUG 2022".to_owned(),
            status: StatusTag::Complete,
            lang: LangTag::Rust,
            stack: vec![StackTag::Wasm, StackTag::Dioxus],
            links: vec![
                ProjectLink::GitHub("https://github.com/lectromoe/Pomodoro".to_owned()),
                ProjectLink::Mirror("https://lectromoe.github.io/Pomodoro/".to_owned()),

                // TODO: Update when hosted on lectro.moe
                ProjectLink::Use("https://lectromoe.github.io/Pomodoro/".to_owned()), 
            ],
        },
        Project {
            name: "lectro.moe".to_owned(),
            description: "a personal website".to_owned(),
            date: "SEP 2022".to_owned(),
            status: StatusTag::Complete,
            lang: LangTag::Rust,
            stack: vec![StackTag::Wasm, StackTag::Yew],
            links: vec![
                ProjectLink::GitHub("https://github.com/lectromoe/lectro.moe".to_owned()),
                ProjectLink::Mirror("https://lectromoe.github.io/lectro.moe/".to_owned()),
            ],
        },
    ];

    let projects: Html = projects.into_iter().collect();

    html! {
        <div id="projects" class="h-96 h-screen mb-52">
            <h1 class={format!("{} {}", SECTION_TITLE, PROJECTS_GRADIENT)}>{ "Projects" }</h1>
            <div class="relative shadow-md sm:rounded-lg">
                <table class="relative w-max text-sm text-left table-fixed overflow-visible ">
                    <tbody>
                        { projects }
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[function_component(Articles)]
pub fn articles() -> Html {
    html! {
        <div id="articles" class="h-96 h-screen mb-52">
            <h1 class={format!("{} {}", SECTION_TITLE, ARTICLES_GRADIENT)}>{ "Articles" }</h1>
        </div>
    }
}

#[function_component(Contacts)]
pub fn contacts() -> Html {
    html! {
        <div id="contacts" class="h-96 h-screen">
            <h1 class={format!("{} {}", SECTION_TITLE, CONTACTS_GRADIENT)}>{ "Contacts" }</h1>
            <h2 class="text-2xl">
                <span class="contacts_gradient">{ "Telegram: " }</span>
                <a href="https://t.me/lectromoe">{ "@lectromoe" }</a>
            </h2>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="font-['Roboto'] w-max h-max">
            <bg class="h-full w-full fixed bg-fixed bg-gradient-to-b from-black to-slate-900"> </bg>
            <content class="absolute w-full">
                <div class="flex flex-col w-3/4 xl:w-1/2 mx-auto text-stone-200 ">
                    <div class="flex flex-col h-screen">
                        <TopBar />
                        <Status />
                    </div>
                    <Projects />
                    <Articles />
                    <Contacts />
                </div>
            </content>
        </main>
    }
}

fn main() {
    yew::start_app::<App>();
}
