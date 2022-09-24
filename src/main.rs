use yew::prelude::*;

#[function_component(TopBar)]
pub fn top_bar() -> Html {
    html! {
        <div>
            { "Top Bar" }
        </div>
    }
}

#[function_component(Projects)]
pub fn prjects() -> Html {
    html! {
        <div>
            { "Projects" }
        </div>
    }
}

#[function_component(Articles)]
pub fn articles() -> Html {
    html! {
        <div>
            { "Articles" }
        </div>
    }
}

#[function_component(Contacts)]
pub fn contacts() -> Html {
    html! {
        <div>
            { "Contacts" }
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <TopBar />
            <Projects />
            <Articles />
            <Contacts />
        </main>
    }
}

fn main() {
    yew::start_app::<App>();
}
