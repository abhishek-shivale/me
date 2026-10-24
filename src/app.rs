use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::components::{self, Theme};
use crate::pages;
use crate::utils::resume::Resume;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/blog/:slug")]
    Post { slug: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route, resume: &Resume) -> Html {
    match route {
        Route::Home => pages::home::home(resume),
        Route::Blog => pages::blog::blog(),
        Route::Post { slug } => pages::blog::post(&slug),
        Route::NotFound => html! { <p class="section-heading">{ "Page not found" }</p> },
    }
}

#[component]
fn Layout() -> Html {
    let resume = Resume::parse().expect("invalid Resume.toml");
    let name = resume.personal.name.clone();
    let source_url = resume.personal.source_url.clone();
    let nav_items = resume.navbar.clone();
    html! {
        <ContextProvider<Theme> context={Theme::default()}>
            <div class="site-container">
                <components::navbar::Navbar name={name.clone()} items={nav_items} />
                <Switch<Route> render={move |route| switch(route, &resume)} />
                <components::footer::Footer {name} {source_url} />
            </div>
        </ContextProvider<Theme>>
    }
}

#[component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Layout />
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq)]
pub struct ServerAppProps {
    pub path: AttrValue,
}

#[component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history.push(props.path.as_str());
    html! {
        <Router {history}>
            <Layout />
        </Router>
    }
}
