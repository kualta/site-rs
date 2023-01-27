use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DataSchema {
    pub projects: Vec<Entry>,
    pub articles: Vec<Entry>,
    pub contacts: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub description: String,
    pub link: String,
    pub theme: Option<String>,
    pub date: Option<String>,
    pub stack: Option<String>,
    pub status: Option<String>,
}
