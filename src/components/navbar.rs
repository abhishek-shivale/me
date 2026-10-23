use yew::prelude::*;

use crate::utils::resume::NavItem;

const THEME_KEY: &str = "theme";

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub name: AttrValue,
    pub items: Vec<NavItem>,
}

#[component]
pub fn Navbar(props: &NavbarProps) -> Html {
    html! {
        <div class="heading">
            <a class="heading-name" href="/">{ &props.name }</a>
            <nav class="heading-nav">
                { for props.items.iter().map(|item| html! {
                    <a class="heading-nav-element" href={item.href.clone()}>
                        <i class={item.icon.class()} aria-hidden="true"></i>
                        { &item.label }
                    </a>
                }) }
                <ThemeToggle />
            </nav>
        </div>
    }
}

fn initial_light() -> bool {
    let Some(window) = web_sys::window() else { return false };
    let saved = window
        .local_storage()
        .ok()
        .flatten()
        .and_then(|s| s.get_item(THEME_KEY).ok().flatten());
    
    match saved.as_deref() {
        Some("light") => true,
        Some("dark") => false,
        _ => window
            .match_media("(prefers-color-scheme: light)")
            .ok()
            .flatten()
            .is_some_and(|m| m.matches()),
    }
}

fn apply_theme(light: bool) {
    let Some(window) = web_sys::window() else { return };
    let name = if light { "light" } else { "dark" };
    if let Some(root) = window.document().and_then(|d| d.document_element()) {
        let _ = root.set_attribute("data-theme", name);
    }
    
    if let Ok(Some(storage)) = window.local_storage() {
        let _ = storage.set_item(THEME_KEY, name);
    }
}

#[component]
fn ThemeToggle() -> Html {
    let light = use_state(initial_light);
    use_effect_with(*light, |&light| apply_theme(light));
    let onclick = {
        let light = light.clone();
        Callback::from(move |_| light.set(!*light))
    };
    let (icon, label) = if *light {
        ("fa-solid fa-moon", "Switch to dark mode")
    } else {
        ("fa-regular fa-sun", "Switch to light mode")
    };
    html! {
        <button type="button" class="theme-toggle" aria-label={label} title={label} {onclick}>
            <i class={icon} aria-hidden="true"></i>
        </button>
    }
}
