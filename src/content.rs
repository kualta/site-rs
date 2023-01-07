use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub name: String,
    pub date: String,
    pub stack: Stack,
    pub status: Status,
    pub description: String,
    pub language: Language,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Stack(pub String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Link(pub String, pub String);

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
