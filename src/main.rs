#![allow(clippy::let_unit_value)]

use gloo::timers::callback::Timeout;
use rand::prelude::*;
use web_sys::HtmlAudioElement;
use yew::prelude::*;
mod project;
use project::*;

const PROJECTS_GRADIENT: &str =
    "bg-gradient-to-r from-rose-400 to-fuchsia-500 text-transparent bg-clip-text";
const ARTICLES_GRADIENT: &str =
    "bg-gradient-to-r from-pink-300 to-indigo-400 text-transparent bg-clip-text";
const CONTACTS_GRADIENT: &str =
    "bg-gradient-to-r from-green-200 via-emerald-200  to-green-300 text-transparent bg-clip-text";
const RUST_GRADIENT: &str =
    "bg-gradient-to-r from-red-200 via-red-300 to-yellow-200 text-transparent bg-clip-text ";
const LECTRO_GRADIENT: &str = "bg-gradient-to-r from-red-200 via-red-300 to-yellow-200";

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
                <a href="#projects" class="transition-all hover:text-fuchsia-300">{ "projects" }</a>
                <a href="#articles" class="transition-all hover:text-indigo-200">{ "articles" }</a>
                <a href="#contacts" class="transition-all hover:text-green-200">{ "contacts" }</a>
            </div>
        </div>
    }
}

#[function_component(Status)]
pub fn status() -> Html {
    html! {
        <div id="status" class="roboto-mono mono flex flex-grow flex-col text-3xl text-left place-content-center gap-8 p-4">
            <div>
                { "> I write Engines, Systems and Tools " }
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
    let projects = vec![Project {
        name: "Fuji Bot".to_owned(),
        description: "hardcore hypercasual game".to_owned(),
        date: "SEP 2019".to_owned(),
        status: StatusTag::Released,
        lang: LangTag::Cs,
        stack: vec![StackTag::Unity],
        links: vec![ProjectLink::GitHub(
            "https://github.com/lectromoe/fuji-bot".to_owned(),
        )],
    }];

    let odd_row_style = "bg-gray-900 border-gray-700";
    let even_row_style = "bg-gray-800 border-gray-700";
    let column_style = "py-4 px-6 font-medium whitespace-nowrap";
    let projects: Html = projects
        .iter()
        .enumerate()
        .map(|(i, proj)| {
            let row_style = if i % 2 == 0 {
                even_row_style
            } else {
                odd_row_style
            };

            let stack_elems = proj
                .stack
                .iter()
                .map(|elem| Html::from(*elem))
                .collect::<Html>();

            let links = proj
                .links
                .iter()
                .map(|link| Html::from(link.clone()))
                .collect::<Html>();

            html! {
                <tr class={row_style}>
                    <th scope="row" class={column_style}>{&proj.name}</th>
                    <th scope="row" class={column_style}>{Html::from(proj.status)}</th>
                    <th scope="row" class={column_style}>{&proj.date}</th>
                    <th scope="row" class={column_style}>{Html::from(proj.lang)}</th>
                    <th scope="row" class={column_style}>{stack_elems}</th>
                    <th scope="row" class={column_style}>{links}</th>
                    <th scope="row" class={column_style}>{&proj.description}</th>
                </tr>
            }
        })
        .collect();

    html! {
        <div id="projects" class="h-96 h-screen">
            <h1 class={format!("text-6xl font-bold p-4 inline-block {}", PROJECTS_GRADIENT)}>{ "projects" }</h1>
            <div class="overflow-x-auto relative shadow-md sm:rounded-lg">
                <table class="w-full text-sm text-left">
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
        <div id="articles" class="h-96 h-screen">
            <h1 class={format!("text-6xl font-bold p-4 inline-block {}", ARTICLES_GRADIENT)}>{ "articles" }</h1>
        </div>
    }
}

#[function_component(Contacts)]
pub fn contacts() -> Html {
    html! {
        <div id="contacts" class="h-96 h-screen">
            <h1 class={format!("text-6xl font-bold p-4 inline-block {}", CONTACTS_GRADIENT)}>{ "contacts" }</h1>
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
