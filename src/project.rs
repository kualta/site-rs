use yew::{html, Html};
use yew_feather::{
    arrow_up_right::ArrowUpRight, command::Command, external_link::ExternalLink, github::Github,
    gitlab::Gitlab, globe::Globe, package::Package, play::Play,
};

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
        let stack_elems = project
            .stack
            .iter()
            .map(|elem| {
                html! {
                    <span>
                        <span> {", "} </span>
                        {Html::from(*elem)}
                    </span>
                }
            })
            .collect::<Html>();

        let links = project
            .links
            .iter()
            .map(|link| Html::from(link.clone()))
            .collect::<Html>();

        let row_style = "border-gray-800 border-y-2 p-4 hidden";
        let column_style = "py-4 px-6 whitespace-nowrap";
        html! {
            <tr class={column_style}>
                <th scope="row" class={format!("{} md:table-cell", row_style)}>{Html::from(project.status)}</th>
                <th scope="row" class={format!("{} lg:table-cell", row_style)}>{&project.date}</th>
                <th scope="row" class={format!("{} !table-cell", row_style)}>{&project.name}</th>
                <th scope="row" class={format!("{} sm:table-cell", row_style)}>{&project.description}</th>
                <th scope="row" class={format!("{} lg:table-cell", row_style)}>{Html::from(project.lang)} {stack_elems}</th>
                <th scope="row" class={format!("{} !table-cell", row_style)}>{links}</th>
            </tr>
        }
    }
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
            <span class={style}>
                {text}
            </span>
        }
    }
}

#[derive(Clone, Copy)]
pub enum StatusTag {
    Archived,
    Complete,
    Working,
}
impl From<StatusTag> for Html {
    fn from(tag: StatusTag) -> Self {
        let (style, text) = match tag {
            StatusTag::Archived => ("text-gray-400", "Archived"),
            StatusTag::Complete => ("text-pink-500", "Complete"),
            StatusTag::Working => ("text-red-400", "In Development"),
        };

        html! {
            <span class={style}>
                {text}
            </span>
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
            <span class="">
                {text}
            </span>
        }
    }
}

#[derive(Clone)]
pub enum ProjectLink {
    Use(String),
    Steam(String),
    GitHub(String),
    GitLab(String),
    Website(String),
    Mirror(String),
    AppStore(String),
    Cratesio(String),
    GooglePlay(String),
}
impl From<ProjectLink> for Html {
    #[rustfmt::skip]
    fn from(tag: ProjectLink) -> Self {
        let icon = "inline m-1 mr-4";
        let size = "16";
        let (text, link, icon) = match tag {
            ProjectLink::Use(link)        => ("Use",         link, html! {<ArrowUpRight size={size} class={icon} />}),
            ProjectLink::Steam(link)      => ("Steam",       link, html! {<ExternalLink size={size} class={icon} />}),
            ProjectLink::GitHub(link)     => ("GitHub",      link, html! {<Github       size={size} class={icon} />}),
            ProjectLink::Mirror(link)     => ("Mirror",      link, html! {<ExternalLink size={size} class={icon} />}),
            ProjectLink::GitLab(link)     => ("GitLab",      link, html! {<Gitlab       size={size} class={icon} />}),
            ProjectLink::Website(link)    => ("Site",        link, html! {<Globe        size={size} class={icon} />}),
            ProjectLink::AppStore(link)   => ("AppStore",    link, html! {<Command      size={size} class={icon} />}),
            ProjectLink::Cratesio(link)   => ("Crates.io",   link, html! {<Package      size={size} class={icon} />}),
            ProjectLink::GooglePlay(link) => ("Google Play", link, html! {<Play         size={size} class={icon} />}),
        };

        html! {
            <span class="">
                <a href={link} class="underline"> {text} {icon} </a>
            </span>
        }
    }
}
