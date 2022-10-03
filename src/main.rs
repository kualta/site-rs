#![allow(clippy::let_unit_value)]
#![feature(custom_inner_attributes)]
#[rustfmt::skip]
mod content;

use content::*;
use gloo::timers::callback::Timeout;
use rand::prelude::*;
use web_sys::HtmlAudioElement;
use yew::prelude::*;

const PUBLIC_URL: &str = "/lectro.moe/";

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
                let path = PUBLIC_URL.to_owned() + "assets/sound/medal_click_rare.wav";
                let _ = HtmlAudioElement::new_with_src(&path)
                    .expect("Failed to load resource")
                    .play();
                is_active.set(false);
                animation.set("flip");
            })
        } else {
            Callback::from(move |_| {
                let path = PUBLIC_URL.to_owned() + "assets/sound/medal_click.wav";
                let _ = HtmlAudioElement::new_with_src(&path)
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
            <div class="flex flex-row items-center">
                <Avatar />
                <h3 class={format!("p-8 text-transparent bg-clip-text text-3xl font-bold {}", LECTRO_GRADIENT)}>
                    { "lectro.moe" }
                </h3>
            </div>
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
                { "> I write "}
                <a href="#projects" class={format!("inline-block {}", GAMES_GRADIENT)}>{ "Games" }</a>
                {", Engines and Tools " }
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

#[derive(PartialEq, Properties)]
pub struct ProjectsProps {
    content: Html,
}

#[function_component(Projects)]
pub fn projects(props: &ProjectsProps) -> Html {
    html! {
        <div id="projects" class="mb-52">
            <h1 class={format!("{} {}", SECTION_TITLE, PROJECTS_GRADIENT)}>{ "Projects" }</h1>
            <div class="shadow-md sm:rounded-lg">
                <table class="w-max text-sm text-left table-fixed overflow-visible ">
                    <tbody>
                        { props.content.clone() }
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct ArticlesProps {
    content: Html,
}

#[function_component(Articles)]
pub fn articles(props: &ArticlesProps) -> Html {
    html! {
        <div id="articles" class="mb-52">
            <h1 class={format!("{} {}", SECTION_TITLE, ARTICLES_GRADIENT)}>{ "Articles" }</h1>
            <div class="shadow-md sm:rounded-lg">
                <table class="w-max text-sm text-left table-fixed overflow-visible ">
                    <tbody>
                        { props.content.clone() }
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct ContactsProps {
    content: Html,
}

#[function_component(Contacts)]
pub fn contacts(props: &ContactsProps) -> Html {
    html! {
        <div id="contacts" class="mb-96">
            <h1 class={format!("{} {}", SECTION_TITLE, CONTACTS_GRADIENT)}>{ "Contacts" }</h1>
            <div class="shadow-md sm:rounded-lg">
                <table class="w-max text-sm text-left table-fixed overflow-visible ">
                    <tbody>
                        { props.content.clone() }
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let content = use_state(Content::default);
    let projects: Html = content.projects.clone().into_iter().collect();
    let articles: Html = content.articles.clone().into_iter().collect();
    let contacts: Html = content.contacts.clone().into_iter().collect();

    html! {
        <main class="roboto-mono w-max h-max min-w-fit">
            <bg class="h-full w-full fixed bg-fixed bg-gradient-to-b from-black to-slate-900"> </bg>
            <content class="absolute w-full">
                <div class="flex flex-col w-3/4 xl:w-1/2 mx-auto text-stone-200 ">
                    <div class="flex flex-col h-screen">
                        <TopBar />
                        <Status />
                    </div>
                    <Projects content={projects} />
                    <Articles content={articles} />
                    <Contacts content={contacts} />
                </div>
            </content>
        </main>
    }
}

fn main() {
    yew::start_app::<App>();
}
