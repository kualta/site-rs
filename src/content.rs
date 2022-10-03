use yew::{html, Html, Properties};
use yew_feather::{
    command::Command, download::Download, external_link::ExternalLink, file_text::FileText,
    github::Github, gitlab::Gitlab, globe::Globe, mail::Mail, package::Package, phone::Phone,
    play::Play, send::Send, twitter::Twitter,
};

pub const CONTACTS_GRADIENT: &str =
    "bg-gradient-to-r from-green-200 via-emerald-300 to-green-300 text-transparent bg-clip-text";
pub const GAMES_GRADIENT: &str =
    "bg-gradient-to-r from-fuchsia-300 via-fuchsia-300 to-pink-300 text-transparent bg-clip-text";
pub const RUST_GRADIENT: &str =
    "bg-gradient-to-r from-red-200 via-red-300 to-yellow-200 text-transparent bg-clip-text ";
pub const PROJECTS_GRADIENT: &str =
    "bg-gradient-to-r from-rose-400 to-fuchsia-500 text-transparent bg-clip-text";
pub const ARTICLES_GRADIENT: &str =
    "bg-gradient-to-r from-pink-300 to-indigo-400 text-transparent bg-clip-text";
pub const LECTRO_GRADIENT: &str = "bg-gradient-to-r from-red-200 via-red-300 to-yellow-200";
pub const SECTION_TITLE: &str = "text-5xl font-bold p-4 mb-8 roboto-mono inline-block";

#[derive(Clone, PartialEq)]
pub struct Content {
    pub projects: Vec<Project>,
    pub articles: Vec<Article>,
    pub contacts: Vec<Contact>,
}

impl Default for Content {
    fn default() -> Self {
        Content {
            projects:  vec![
                Project {
                    name: "Fuji Bot".to_owned(),
                    description: "a hardcore hypercasual game".to_owned(),
                    date: "SEP 2019".to_owned(),
                    status: StatusTag::Complete,
                    lang: LangTag::Cs,
                    stack: vec![StackTag::Unity],
                    links: vec![
                        ContentLink::GitHub("https://github.com/lectromoe/fuji-bot".to_owned()),
                        ContentLink::GooglePlay("https://play.google.com/store/apps/details?id=com.lectroMathew.FujiBot&hl=en&gl=US".to_owned()),
                     ],
                },
                Project {
                    name: "Core Engine".to_owned(),
                    description: "a neat game engine".to_owned(),
                    date: "JAN 2021".to_owned(),
                    status: StatusTag::Archived,
                    lang: LangTag::Cpp,
                    stack: vec![StackTag::Core],
                    links: vec![ContentLink::GitHub(
                        "https://github.com/lectromoe/Core".to_owned(),
                    )],
                },
                Project {
                    name: "Hemi Typer".to_owned(),
                    description: "an experimental typing tutor".to_owned(),
                    date: "MAR 2022".to_owned(),
                    status: StatusTag::Complete,
                    lang: LangTag::Rust,
                    stack: vec![StackTag::Wasm, StackTag::Dioxus],
                    links: vec![
                        ContentLink::GitHub("https://github.com/lectromoe/HemiTyper".to_owned()),

                        // TODO: Update when hosted on lectro.moe
                        ContentLink::Use("https://lectromoe.github.io/HemiTyper/".to_owned()), 
                    ],
                },
                Project {
                    name: "Pomodoro".to_owned(),
                    description: "a simple pomo timer".to_owned(),
                    date: "AUG 2022".to_owned(),
                    status: StatusTag::Complete,
                    lang: LangTag::Rust,
                    stack: vec![StackTag::Wasm, StackTag::Dioxus],
                    links: vec![
                        ContentLink::GitHub("https://github.com/lectromoe/Pomodoro".to_owned()),

                        // TODO: Update when hosted on lectro.moe
                        ContentLink::Use("https://lectromoe.github.io/Pomodoro/".to_owned()), 
                    ],
                },
                Project {
                    name: "lectro.moe".to_owned(),
                    description: "a personal website".to_owned(),
                    date: "SEP 2022".to_owned(),
                    status: StatusTag::Complete,
                    lang: LangTag::Rust,
                    stack: vec![StackTag::Wasm, StackTag::Yew],
                    links: vec![
                        ContentLink::GitHub("https://github.com/lectromoe/lectro.moe".to_owned()),
                        ContentLink::Mirror("https://lectromoe.github.io/lectro.moe/".to_owned()),
                    ],
                },
            ],
            articles: vec![
                Article {
                    name: "Fair Play: The art of online games balance".to_owned(),
                    date: "JUL 2021".to_owned(),
                    tag: ArticleTag::GameDesign,
                    links: vec![
                        ContentLink::Download("https://lectro.medium.com/fair-play-the-art-of-online-games-balance-658a2cc39ea3".to_owned()),
                        ContentLink::Mirror("https://lectro.medium.com/fair-play-the-art-of-online-games-balance-658a2cc39ea3".to_owned()),
                        ContentLink::Read("https://lectro.medium.com/fair-play-the-art-of-online-games-balance-658a2cc39ea3".to_owned()),
                    ],
                }
            ],
            contacts: vec![
                Contact {
                    name: "Telegram".to_owned(),
                    handle: "@lectromoe".to_owned(),
                    link: "https://t.me/lectromoe".to_owned(),
                    tag: ContactTag::Telegram,
                },
                Contact {
                    name: "GitHub".to_owned(),
                    handle: "@lectromoe".to_owned(),
                    link: "https://github.com/lectromoe".to_owned(),
                    tag: ContactTag::GitHub,
                },
                Contact {
                    name: "Twitter".to_owned(),
                    handle: "@lectromoe".to_owned(),
                    link: "https://twitter.com/lectromoe".to_owned(),
                    tag: ContactTag::Twitter,
                },
                Contact {
                    name: "HR".to_owned(),
                    handle: "jobs@lectro.moe".to_owned(),
                    link: "mailto:jobs@lectro.moe".to_owned(),
                    tag: ContactTag::Email,
                },
                Contact {
                    name: "Feedback".to_owned(),
                    handle: "contact@lectro.moe".to_owned(),
                    link: "mailto:contact@lectro.moe".to_owned(),
                    tag: ContactTag::Email,
                },
                // I have removed the discord contact from the public eye for now,
                // but if you see this you might be special! So I left it here for you.
                // Contact {
                //     name: "Discord".to_owned(),
                //     handle: "lectromoe#6858".to_owned(),
                //     link: "https://discordapp.com/users/964306690889637900".to_owned(),
                //     tag: ContactTag::Discord,
                // },
            ]
        }
    }
}

#[derive(PartialEq, Clone)]
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

        let row_style = "border-gray-800 border-y-2 p-4 hidden font-medium";
        let column_style = "py-4 px-6 whitespace-nowrap";
        html! {
            <tr class={column_style}>
                <th scope="row" class={format!("{} md:table-cell", row_style)}> {Html::from(project.status)} </th>
                <th scope="row" class={format!("{} lg:table-cell", row_style)}> {&project.date}              </th>
                <th scope="row" class={format!("{} !table-cell !font-bold",   row_style)}> {&project.name}   </th>
                <th scope="row" class={format!("{} sm:table-cell text-stone-400", row_style)}>
                                                                                {&project.description}       </th>
                <th scope="row" class={format!("{} lg:table-cell", row_style)}> {Html::from(project.lang)}
                                                                                {stack_elems}                </th>
                <th scope="row" class={format!("{} !table-cell",   row_style)}> {links}                      </th>
            </tr>
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Article {
    pub name: String,
    pub date: String,
    pub tag: ArticleTag,
    pub links: Vec<ContentLink>,
}

impl From<Article> for Html {
    fn from(article: Article) -> Self {
        let row_style = "border-gray-800 border-y-2 p-4 hidden font-medium";
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

#[derive(PartialEq, Properties, Clone)]
pub struct Contact {
    pub name: String,
    pub handle: String,
    pub link: String,
    pub tag: ContactTag,
}

impl From<Contact> for Html {
    fn from(contact: Contact) -> Self {
        let row_style = "border-gray-800 border-y-2 p-4 font-medium";
        let column_style = "py-4 px-6 whitespace-nowrap";

        html! {
            <tr class={column_style}>
                <th scope="row" class={format!("{}", row_style)}> {Html::from(contact.tag)} </th>
                <th scope="row" class={format!("{} text-green-200",   row_style)}> {contact.name} </th>
                <th scope="row" class={format!("{} roboto-mono", row_style)}>
                    <a href={contact.link.to_owned()}> {contact.handle} </a>
                </th>
            </tr>
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ContactTag {
    Email,
    GitHub,
    Telegram,
    Twitter,
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
            ContactTag::GitHub =>   html! {<Github       size={size} class={icon} />},
            ContactTag::Twitter =>  html! {<Twitter      size={size} class={icon} />},
        };

        html! { {icon} }
    }
}

#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy, PartialEq)]
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
            ContentLink::Use(link)        => ("Use",         link, html! {<ExternalLink size={size} class={icon} />}),
            ContentLink::Read(link)       => ("Read",        link, html! {<FileText     size={size} class={icon} />}),
            ContentLink::Steam(link)      => ("Steam",       link, html! {<ExternalLink size={size} class={icon} />}),
            ContentLink::GitHub(link)     => ("Github",      link, html! {<Github       size={size} class={icon} />}),
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
                <a href={link.to_owned()} title={text} class="underline"> {icon} </a>
            </span>
        }
    }
}
