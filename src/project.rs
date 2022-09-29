use yew::Html;

pub struct Project {
    pub name: String,
    pub description: String,
    pub date: String,
    pub status: StatusTag,
    pub lang: LangTag,
    pub stack: Vec<StackTag>,
    pub links: Vec<ProjectLink>,
}

impl From<Project> for Html {
    fn from(project: Project) -> Self {
        todo!()
    }
}

pub enum LangTag {
    C,
    Cs,
    Cpp,
    Rust,
    Python,
}

pub enum StatusTag {
    Archived,
    Released,
    Working,
}

pub enum StackTag {
    Quantum,
    Dioxus,
    Unity,
    Core,
    Bevy,
    Wasm,
    Yew,
}

pub enum ProjectLink {
    Steam(String),
    GitHub(String),
    GitLab(String),
    Website(String),
    AppStore(String),
    Cratesio(String),
    GooglePlay(String),
}
