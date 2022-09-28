use gloo::timers::callback::Timeout;
use rand::prelude::*;
use yew::prelude::*;

const PROJECTS_GRADIENT: &str =
    "bg-gradient-to-r from-rose-400 to-fuchsia-500 text-transparent bg-clip-text";
const ARTICLES_GRADIENT: &str =
    "bg-gradient-to-r from-pink-300 to-indigo-400 text-transparent bg-clip-text";
const CONTACTS_GRADIENT: &str =
    "bg-gradient-to-r from-yellow-100 to-yellow-300 text-transparent bg-clip-text";
const LECTRO_GRADIENT: &str = "bg-gradient-to-r from-red-200 via-red-300 to-yellow-200 ";

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
        let result = rand::thread_rng().gen_range(0..10);
        let animation = animation.clone();

        if !*is_active {
            Callback::from(move |_| {}) 
        } else if result == 0 {
            Callback::from(move |_| {
                is_active.set(false);
                animation.set("flip");
            })
        } else {
            Callback::from(move |_| {
                if *animation == "shake-x" {
                    animation.set("shake-y");
                } else {
                    animation.set("shake-x");
                }
            })
        }
    };
    html! {
        <div class={format!("flex shrink-0 items-center rounded-full p-1 m-4 w-20 {} {}", LECTRO_GRADIENT, *animation)} {onclick}>
            <img src="assets/img/ps2doggy.png" alt="doggy" class="rounded-full "/>
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
            <div class="flex flex-row lg:ml-auto text-xl font-medium p-8 gap-16 hover:text-white">
                <a href="#projects" class="text-white transition-all hover:text-fuchsia-300">{ "projects" }</a>
                <a href="#articles" class="text-white transition-all hover:text-indigo-200">{ "articles" }</a>
                <a href="#contacts" class="text-white transition-all hover:text-yellow-200">{ "contacts" }</a>
            </div>
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
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="font-['Roboto'] w-max h-max">
            <bg class="h-full w-full fixed bg-fixed bg-gradient-to-b from-black to-slate-900"> </bg>
            <content class="absolute w-full">
                <div class="flex flex-col w-3/4 xl:w-1/2 mx-auto text-white ">
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
