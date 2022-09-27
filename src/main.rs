use yew::prelude::*;

const PROJECTS_GRADIENT: &str =
    "bg-gradient-to-r from-rose-400 to-fuchsia-500 text-transparent bg-clip-text";
const ARTICLES_GRADIENT: &str =
    "bg-gradient-to-r from-pink-300 to-indigo-400 text-transparent bg-clip-text";
const CONTACTS_GRADIENT: &str =
    "bg-gradient-to-r from-yellow-100 to-yellow-300 text-transparent bg-clip-text";

#[function_component(TopBar)]
pub fn top_bar() -> Html {
    html! {
        <div class="flex flex-col md:flex-row items-center place-content-between p-4">
            <div class="flex shrink-0 items-center rounded-full bg-gradient-to-r from-red-200 via-red-300 to-yellow-200 p-1 m-4 w-20">
                <img src="assets/img/ps2doggy.png" alt="doggy" class="rounded-full"/>
            </div>
            <h3 class="p-8 text-transparent bg-clip-text text-3xl font-bold bg-gradient-to-r from-red-200 via-red-300 to-yellow-200">
                { "lectro.moe" }
            </h3>
            <div class="flex flex-row text-xl font-bold p-8 gap-16 hover:text-white">
                <a href="#projects" class={PROJECTS_GRADIENT}>{ "projects" }</a>
                <a href="#articles" class={ARTICLES_GRADIENT}>{ "articles" }</a>
                <a href="#contacts" class={CONTACTS_GRADIENT}>{ "contacts" }</a>
            </div>
        </div>
    }
}

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <div id="projects" class="h-96 h-screen">
            <h1 class={format!("{} text-6xl font-bold p-4 inline-block", PROJECTS_GRADIENT)}>{ "projects" }</h1>
        </div>
    }
}

#[function_component(Articles)]
pub fn articles() -> Html {
    html! {
        <div id="articles" class="h-96 h-screen">
            <h1 class={format!("{} text-6xl font-bold p-4 inline-block", ARTICLES_GRADIENT)}>{ "articles" }</h1>
        </div>
    }
}

#[function_component(Contacts)]
pub fn contacts() -> Html {
    html! {
        <div id="contacts" class="h-96 h-screen">
            <h1 class={format!("{} text-6xl font-bold p-4 inline-block", CONTACTS_GRADIENT)}>{ "contacts" }</h1>
        </div>
    }
}

#[function_component(Status)]
pub fn status() -> Html {
    html! {
        <div id="status" class="h-96 h-screen">
            <h1 class="text-6xl font-bold p-4 inline-block">{ "STATUS" }</h1>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="font-['Roboto'] w-max h-max">
            <bg class="h-full w-full fixed bg-fixed bg-gradient-to-b from-black to-slate-900"> </bg>
            <content class="absolute w-full">
                <div class="flex flex-col w-3/4 md:w-3/4 lg:w-1/2 mx-auto text-white ">
                    <TopBar />
                    <Status />
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
