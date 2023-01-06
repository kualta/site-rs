use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub date: String,
    pub status: Status,
    pub language: Language,
    pub tech: Vec<Tech>,
    pub links: Vec<Link>,
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
pub enum Tech {
    Quantum,
    Dioxus,
    Unity,
    Core,
    Bevy,
    Wasm,
    Yew,
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
