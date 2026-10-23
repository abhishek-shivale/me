use yew::prelude::*;

use crate::components::Theme;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub name: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub href: Option<AttrValue>,
}

#[component]
pub fn Button(props: &ButtonProps) -> Html {
    let theme = use_context::<Theme>().expect("no ctx found");
    let style = format!(
        "background: {}; color: {};",
        theme.background,
        theme.foreground
    );
    match &props.href {
        Some(href) => html! {
            <a href={href.clone()} class={props.class.clone()} {style}>
                { &props.name }
            </a>
        },
        None => html! {
            <span class={props.class.clone()} {style}>
                { &props.name }
            </span>
        },
    }
}
