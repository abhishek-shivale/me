mod utils;
mod pages;
mod components;

use yew::prelude::*;
use yew_router::prelude::*;

use utils::resume::Resume;

#[derive(Clone, PartialEq, Routable)]
enum Route {
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
fn App() -> Html {
    let resume = Resume::parse().expect("invalid Resume.toml");
    let name = resume.personal.name.clone();
    let source_url = resume.personal.source_url.clone();
    let nav_items = resume.navbar.clone();
    html! {
        <BrowserRouter>
            <ContextProvider<components::Theme> context={components::Theme::default()}>
                <div class="site-container">
                    <components::navbar::Navbar name={name.clone()} items={nav_items} />
                    <Switch<Route> render={move |route| switch(route, &resume)} />
                    <components::footer::Footer {name} {source_url} />
                </div>
            </ContextProvider<components::Theme>>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
