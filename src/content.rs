use yew::{html, Html};
use yew_feather::{
    arrow_up_right::ArrowUpRight, command::Command, download::Download,
    external_link::ExternalLink, file_text::FileText, github::Github, gitlab::Gitlab, globe::Globe,
    mail::Mail, package::Package, phone::Phone, play::Play, send::Send,
};

pub struct Project {
    pub name: String,
    pub description: String,
    pub date: String,
    pub status: StatusTag,
    pub lang: LangTag,
    pub stack: Vec<StackTag>,
    pub links: Vec<ContentLink>,
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
                <th scope="row" class={format!("{} md:table-cell", row_style)}> {Html::from(project.status)} </th>
                <th scope="row" class={format!("{} lg:table-cell", row_style)}> {&project.date}              </th>
                <th scope="row" class={format!("{} !table-cell",   row_style)}> {&project.name}              </th>
                <th scope="row" class={format!("{} sm:table-cell", row_style)}> {&project.description}       </th>
                <th scope="row" class={format!("{} lg:table-cell", row_style)}> {Html::from(project.lang)}
                                                                                {stack_elems}                </th>
                <th scope="row" class={format!("{} !table-cell",   row_style)}> {links}                      </th>
            </tr>
        }
    }
}

pub struct Article {
    pub name: String,
    pub date: String,
    pub tag: ArticleTag,
    pub links: Vec<ContentLink>,
}

impl From<Article> for Html {
    fn from(article: Article) -> Self {
        let row_style = "border-gray-800 border-y-2 p-4 hidden";
        let column_style = "py-4 px-6 whitespace-nowrap";

        let links = article
            .links
            .iter()
            .map(|link| Html::from(link.clone()))
            .collect::<Html>();

        html! {
            <tr class={column_style}>
                <th scope="row" class={format!("{} md:table-cell", row_style)}> {Html::from(article.tag)} </th>
                <th scope="row" class={format!("{} lg:table-cell", row_style)}> {article.date}            </th>
                <th scope="row" class={format!("{} !table-cell",   row_style)}> {article.name}            </th>
                <th scope="row" class={format!("{} !table-cell",   row_style)}> {links}                   </th>
            </tr>
        }
    }
}

pub struct Contact {
    pub name: String,
    pub handle: String,
    pub link: String,
    pub tag: ContactTag,
}

impl From<Contact> for Html {
    fn from(contact: Contact) -> Self {
        let row_style = "border-gray-800 border-y-2 p-4";
        let column_style = "py-4 px-6 whitespace-nowrap";

        html! {
            <tr class={column_style}>
                <th scope="row" class={format!("{}", row_style)}> {Html::from(contact.tag)} </th>
                <th scope="row" class={format!("{} text-green-200",   row_style)}> {contact.name} </th>
                <th scope="row" class={format!("{} roboto-mono font-medium", row_style)}>
                    <a href={contact.link}> {contact.handle} </a>
                </th>
            </tr>
        }
    }
}

#[derive(Clone, Copy)]
pub enum ContactTag {
    Email,
    GitHub,
    Telegram,
    Discord,
    Other,
}

impl From<ContactTag> for Html {
    #[rustfmt::skip]
    fn from(tag: ContactTag) -> Self {
        let icon = "inline m-1 mr-4";
        let size = "16";
        let icon = match tag {
            ContactTag::Email =>    html! {<Mail         size={size} class={icon} />},
            ContactTag::Telegram => html! {<Send         size={size} class={icon} />},
            ContactTag::Discord =>  html! {<Phone        size={size} class={icon} />},
            ContactTag::Other =>    html! {<ExternalLink size={size} class={icon} />},
            ContactTag::GitHub =>   html! {<Github       size={size} class={icon} />}
        };

        html! { {icon} }
    }
}

#[derive(Clone, Copy)]
pub enum ArticleTag {
    Engineering,
    GameDesign,
    Tutorial,
    Research,
    Quantum,
    Notes,
}
impl From<ArticleTag> for Html {
    fn from(tag: ArticleTag) -> Self {
        let (style, text) = match tag {
            ArticleTag::Engineering => ("text-blue-800", "Engineering"),
            ArticleTag::GameDesign => ("text-blue-400", "Game Design"),
            ArticleTag::Tutorial => ("text-green-200", "Tutorial"),
            ArticleTag::Research => ("text-white", "Research"),
            ArticleTag::Quantum => ("text-blue-200", "Quantum"),
            ArticleTag::Notes => ("text-grey-300", "Notes"),
        };

        html! {
            <span class={style}>
                {text}
            </span>
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

#[derive(Clone, PartialEq, Eq)]
pub enum ContentLink {
    Use(String),
    Read(String),
    Steam(String),
    GitHub(String),
    GitLab(String),
    Website(String),
    Mirror(String),
    Download(String),
    AppStore(String),
    Cratesio(String),
    GooglePlay(String),
}
impl From<ContentLink> for Html {
    #[rustfmt::skip]
    fn from(tag: ContentLink) -> Self {
        let icon = "inline m-1 mr-4";
        let size = "16";
        let (text, link, icon) = match tag {
            ContentLink::Use(link)        => ("Use",         link, html! {<ArrowUpRight size={size} class={icon} />}),
            ContentLink::Read(link)       => ("Read",        link, html! {<FileText     size={size} class={icon} />}),
            ContentLink::Steam(link)      => ("Steam",       link, html! {<ExternalLink size={size} class={icon} />}),
            ContentLink::GitHub(link)     => ("GitHub",      link, html! {<Github       size={size} class={icon} />}),
            ContentLink::Mirror(link)     => ("Mirror",      link, html! {<ExternalLink size={size} class={icon} />}),
            ContentLink::GitLab(link)     => ("GitLab",      link, html! {<Gitlab       size={size} class={icon} />}),
            ContentLink::Website(link)    => ("Site",        link, html! {<Globe        size={size} class={icon} />}),
            ContentLink::Download(link)   => ("Download",    link, html! {<Download     size={size} class={icon} />}),
            ContentLink::AppStore(link)   => ("AppStore",    link, html! {<Command      size={size} class={icon} />}),
            ContentLink::Cratesio(link)   => ("Crates.io",   link, html! {<Package      size={size} class={icon} />}),
            ContentLink::GooglePlay(link) => ("Google Play", link, html! {<Play         size={size} class={icon} />}),
        };

        html! {
            <span class="">
                <a href={link} class="underline"> {text} {icon} </a>
            </span>
        }
    }
}
