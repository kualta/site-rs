use yew::{html, Html};

pub struct Project {
    pub name: String,
    pub description: String,
    pub date: String,
    pub status: StatusTag,
    pub lang: LangTag,
    pub stack: Vec<StackTag>,
    pub links: Vec<ProjectLink>,
}

#[derive(Clone, Copy)]
pub enum LangTag {
    C,
    Cs,
    Cpp,
    Rust,
    Python,
}
impl From<LangTag> for Html {
    fn from(tag: LangTag) -> Self {
        let (style, text) = match tag {
            LangTag::C => ("text-blue-300", "C"),
            LangTag::Cs => ("text-blue-200", "C#"),
            LangTag::Cpp => ("text-blue-400", "C++"),
            LangTag::Rust => ("text-orange-300", "Rust"),
            LangTag::Python => ("text-yellow-200", "Python"),
        };

        html! {
            <div class={style}>
                {text}
            </div>
        }
    }
}

#[derive(Clone, Copy)]
pub enum StatusTag {
    Archived,
    Released,
    Working,
}
impl From<StatusTag> for Html {
    fn from(tag: StatusTag) -> Self {
        let (style, text) = match tag {
            StatusTag::Archived => ("text-grey-300", "Archived"),
            StatusTag::Released => ("text-pink-500", "Released"),
            StatusTag::Working => ("text-red-400", "In Development"),
        };

        html! {
            <div class={style}>
                {text}
            </div>
        }
    }
}

#[derive(Clone, Copy)]
pub enum StackTag {
    Quantum,
    Dioxus,
    Unity,
    Core,
    Bevy,
    Wasm,
    Yew,
}
impl From<StackTag> for Html {
    fn from(tag: StackTag) -> Self {
        let text = match tag {
            StackTag::Quantum => "Quantum",
            StackTag::Dioxus => "Dioxus",
            StackTag::Unity => "Unity",
            StackTag::Core => "Core",
            StackTag::Bevy => "Bevy",
            StackTag::Wasm => "WASM",
            StackTag::Yew => "Yew",
        };

        html! {
            <div class="">
                {text}
            </div>
        }
    }
}

#[derive(Clone)]
pub enum ProjectLink {
    Steam(String),
    GitHub(String),
    GitLab(String),
    Website(String),
    AppStore(String),
    Cratesio(String),
    GooglePlay(String),
}
impl From<ProjectLink> for Html {
    fn from(tag: ProjectLink) -> Self {
        let (text, link) = match tag {
            ProjectLink::Steam(link) => ("Steam", link),
            ProjectLink::GitHub(link) => ("GitHub", link),
            ProjectLink::GitLab(link) => ("GitLab", link),
            ProjectLink::Website(link) => ("Site", link),
            ProjectLink::AppStore(link) => ("AppStore", link),
            ProjectLink::Cratesio(link) => ("Crates.io", link),
            ProjectLink::GooglePlay(link) => ("GooglePlay", link),
        };

        html! {
            <div class="">
                <a href={link}>{text}</a>
            </div>
        }
    }
}
