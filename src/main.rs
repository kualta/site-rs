#![allow(clippy::let_unit_value)]

use gloo::timers::callback::Timeout;
use rand::prelude::*;
use web_sys::HtmlAudioElement;
use yew::prelude::*;

const PROJECTS_GRADIENT: &str =
    "bg-gradient-to-r from-rose-400 to-fuchsia-500 text-transparent bg-clip-text";
const ARTICLES_GRADIENT: &str =
    "bg-gradient-to-r from-pink-300 to-indigo-400 text-transparent bg-clip-text";
const CONTACTS_GRADIENT: &str =
    "bg-gradient-to-r from-yellow-100 to-yellow-300 text-transparent bg-clip-text";
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
                <a href="#contacts" class="transition-all hover:text-yellow-200">{ "contacts" }</a>
            </div>
        </div>
    }
}

#[function_component(Status)]
pub fn status() -> Html {
    const COMMENT_STYLE: &str = "text-stone-500 font-mono inline-block";
    html! {
        <div id="status" class="flex flex-grow flex-col text-3xl font-medium text-left place-content-around p-4">
            <div>
                { "I write Engines, Systems and Tools " }
                <span class={COMMENT_STYLE}>{ " //  mostly for games" }</span>
            </div>
            <div>
                { "Lately passionate about" } <br />
                <a href="https://www.rust-lang.org/" class={format!("inline-block {}", RUST_GRADIENT)}>{ "Rust" }</a>
                { " and " }
                <a href="https://en.wikipedia.org/wiki/Quantum_computing" class={format!("{}", ARTICLES_GRADIENT)}>{ "Quantum computing " }</a>
                <span class={COMMENT_STYLE}>{ " //  separately so far" }</span>
            </div>
            <div>
                { "Currently not occupied " }
                <span class={COMMENT_STYLE}>{ " //  I could be working for you" }</span>
                <br />
            </div>
            <a href="#contacts" class={format!("text-center !underline decoration-solid {}", CONTACTS_GRADIENT)}>{ "hire me" }</a>
        </div>
    }
}

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <div id="projects" class="h-96 h-screen">
            <h1 class={format!("text-6xl font-bold p-4 inline-block {}", PROJECTS_GRADIENT)}>{ "projects" }</h1>
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
