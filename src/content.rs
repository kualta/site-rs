use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub name: String,
    pub date: String,
    pub stack: String,
    pub description: String,
    pub status: Status,
    pub language: Language,
    pub links: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Article {
    pub name: String,
    pub date: String,
    pub tag: ArticleTag,
    pub links: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Contact {
    pub name: String,
    pub handle: String,
    pub link: String,
    pub tag: ContactTag,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContactTag {
    Email,
    GitHub,
    Telegram,
    Twitter,
    Discord,
    Other,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stack {
    Quantum,
    Dioxus,
    Unity,
    Reactjs,
    Nextjs,
    Core,
    Bevy,
    Wasm,
    Wasi,
    Yew,
}
impl From<&Stack> for LazyNodes<'_, '_> {
    fn from(stack: &Stack) -> Self {
        let class = "";
        match stack {
            Stack::Quantum => rsx! { p { class: class, "Quantum"}},
            Stack::Dioxus => rsx! { p { class: class, "Dioxus"}},
            Stack::Unity => rsx! { p { class: class, "Unity"}},
            Stack::Core => rsx! { p { class: class, "Core"}},
            Stack::Bevy => rsx! { p { class: class, "Bevy"}},
            Stack::Wasm => rsx! { p { class: class, "WASM"}},
            Stack::Wasi => rsx! { p { class: class, "WASI"}},
            Stack::Yew => rsx! { p { class: class, "Yew"}},
            Stack::Reactjs => rsx! { p { class: class, "ReactJS"}},
            Stack::Nextjs => rsx! { p { class: class, "NextJS"}},
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Link {
    Use(String),
    Read(String),
    GitHub(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArticleTag {
    Engineering,
    GameDesign,
    Tutorial,
    Research,
    Quantum,
    Notes,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    C,
    Cs,
    Cpp,
    Rust,
    Python,
    TypeScript,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Archived,
    Complete,
    Working,
}
