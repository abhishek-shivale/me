use yew::prelude::*;

use crate::components::Theme;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub name: AttrValue,
    #[prop_or_default]
    pub class: Classes,
}

#[component]
pub fn Button(props: &ButtonProps) -> Html {
    let theme = use_context::<Theme>().expect("no ctx found");
    html! {
        <button 
            class={props.class.clone()} 
            style={format!(
                "background: {}; color: {};",
                theme.background,
                theme.foreground
            )}
        >
        { &props.name }
        </button>
    }
}
