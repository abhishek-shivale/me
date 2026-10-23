mod utils;
mod pages;
mod components;

use yew::prelude::*;

#[component]
fn App() -> Html {
    // Resume.toml is embedded at compile time, so a parse error is a build-content bug
    let resume = utils::resume::Resume::parse().expect("invalid Resume.toml");
    html! {
        <ContextProvider<components::Theme> context={components::Theme::default()}>
            <div class="site-container">
                <components::navbar::Navbar
                    name={resume.personal.name.clone()}
                    items={resume.navbar.clone()}
                />
                { pages::home::home(&resume) }
            </div>
        </ContextProvider<components::Theme>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
