use yew::prelude::*;

#[function_component(TopBar)]
pub fn top_bar() -> Html {
    html! {
        <div class="flex flex-row items-center place-content-between m-4">
            <div class="flex flex-row items-center">
                <img src="assets/img/ps2doggy.jpg" alt="doggy" class="rounded-full outline outline-offset-4 m-4 w-16 " />
                <h3 class="m-4 text-3xl">{ "lectro.moe" }</h3>
            </div>

            <div class="place-self-end flex flex-row items-center m-8 gap-16">
                <a href="#projects">{ "Projects" }</a>
                <a href="#articles">{ "Articles" }</a>
                <a href="#contacts">{ "Contacts" }</a>
            </div>
        </div>
    }
}

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <div id="projects" class="h-96">
            { "Projects" }
        </div>
    }
}

#[function_component(Articles)]
pub fn articles() -> Html {
    html! {
        <div id="articles" class="h-96">
            { "Articles" }
        </div>
    }
}

#[function_component(Contacts)]
pub fn contacts() -> Html {
    html! {
        <div id="contacts" class="h-96">
            { "Contacts" }
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="font-['Roboto'] bg-black">
            <bg class="h-screen w-screen fixed bg-fixed bg-gradient-to-b from-black to-slate-900"> </bg>
            <content class="absolute w-full">
                <div class="flex flex-col w-1/2 mx-auto text-white ">
                    <TopBar />
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
