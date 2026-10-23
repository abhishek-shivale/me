mod utils;
mod pages;
mod components;

use yew::prelude::*;

#[component]
fn App() -> Html {
    // Resume.toml is embedded at compile time, so a parse error is a build-content bug
    let resume = utils::resume::Resume::parse().expect("invalid Resume.toml");
    html! {
        <div class="site-container">
            { pages::home::home(&resume) }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
