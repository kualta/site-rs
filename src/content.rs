use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Data {
    pub projects: Vec<Project>,
    pub articles: Vec<Article>,
    pub contacts: Vec<Contact>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub name: String,
    pub date: String,
    pub stack: String,
    pub status: String,
    pub description: String,
    pub language: String,
    pub links: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Article {
    pub name: String,
    pub description: String,
    pub theme: String,
    pub links: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Contact {
    pub name: String,
    pub handle: String,
    pub link: String,
}
