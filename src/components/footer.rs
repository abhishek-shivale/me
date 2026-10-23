use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FooterProps {
    pub name: AttrValue,
    pub source_url: AttrValue,
}

#[component]
pub fn Footer(props: &FooterProps) -> Html {
    html! {
        <footer class="site-footer">
            <p class="footer-text">{ &props.name }</p>
            <a class="footer-link" href={props.source_url.clone()} target="_blank" rel="noopener">
                { "Portfolio Source" }
            </a>
        </footer>
    }
}
