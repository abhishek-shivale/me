use yew::prelude::*;

use crate::utils::resume::{Contact, Personal};

#[derive(Properties, PartialEq)]
pub struct HeroProps {
    pub personal: Personal,
    pub contact: Contact,
}

#[component]
pub fn Hero(props: &HeroProps) -> Html {
    let HeroProps { personal, contact } = props;
    let social = &contact.social;
    html! {
        <div class="about-section">
            <p class="heading-location">
                <a href={personal.location_link.clone()} target="_blank" rel="noopener">
                    <i class="fa-solid fa-location-dot" aria-hidden="true"></i>
                    { &personal.location }
                </a>
            </p>
            <p class="about-headline">{ &personal.description }</p>
            <p class="about-paragraph">{ &personal.summary }</p>
            <div class="about-links">
                { for [&social.github, &social.linkedin, &social.x, &social.email].into_iter().map(|link| html! {
                    <a class="about-link" href={link.url.clone()} target="_blank" rel="noopener">
                        { &link.name }
                        <i class={link.icon.class()} aria-hidden="true"></i>
                    </a>
                }) }
            </div>
        </div>
    }
}
