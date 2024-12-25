use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/projects")]
    Projects,
    #[at("/contact")]
    Contact,
    #[at("/*path")]
    Misc,
}

#[derive(PartialEq, Clone)]
pub struct TopBarItem{
    pub title: String,
    pub icon: char,
    pub file: Route,
}

#[derive(PartialEq, Properties)]
pub struct TopBarProps {
    pub items: Vec<TopBarItem>,
}


impl TopBarItem {
    pub fn new(title: String, icon: char, file: Route) -> Self {
        TopBarItem {
            title,
            icon,
            file,
        }
    }
}

impl TopBarProps {
    pub fn new(items: Vec<TopBarItem>) -> Self {
        TopBarProps {
            items,
        }
    }
}

#[function_component(TopBarDiv)]
pub fn top_bar() -> Html {
    let items = vec![
        TopBarItem::new("Home".to_string(), '🏠', Route::Home),
        TopBarItem::new("About Me".to_string(), '📖', Route::About),
        TopBarItem::new("Projects".to_string(), '🛠', Route::Projects),
        TopBarItem::new("Links".to_string(), '🔗', Route::Contact),
    ];

    html! {
        <div class="bg-green-900 flex mx-auto items-center justify-center space-x-10 py-4">
            <TopBar items={items}/>
        </div>
    }
}


#[function_component(TopBar)]
pub fn top_bar_list(TopBarProps {items}: &TopBarProps) -> Html {
    items.iter().map(|item| {
        let route = item.file.clone();
        html! {
                <Link<Route> to={route} classes="bg-gradient-to-t from-emerald-900 to-green-900 shadow-xl rounded-xl px-4 py-1 text-white transition ease-in-out delay-100 hover:scale-110 focus:animate-pulse focus:outline-none focus:ring-2 focus:ring-green-800 hover:shadow-inner hover:from-green-900 hover:to-emerald-900"> {format!("{} {}", item.title, item.icon)} </Link<Route>>
        }
    }).collect()
}

