mod utils;
mod pages;
mod components;

use yew::prelude::*;

#[component]
fn App() -> Html {
    let resume = utils::resume::Resume::parse().expect("invalid Resume.toml");
    html! {
        <ContextProvider<components::Theme> context={components::Theme::default()}>
            <div class="site-container">
                <components::navbar::Navbar
                    name={resume.personal.name.clone()}
                    items={resume.navbar.clone()}
                />
                { pages::home::home(&resume) }
                <components::footer::Footer
                    name={resume.personal.name.clone()}
                    source_url={resume.personal.source_url.clone()}
                />
            </div>
        </ContextProvider<components::Theme>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
