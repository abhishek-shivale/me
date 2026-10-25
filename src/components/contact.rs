use yew::prelude::*;

use crate::utils::resume::Contact;

#[derive(Properties, PartialEq)]
pub struct ContactProps {
    pub contact: Contact,
}

#[component]
pub fn ContactSection(props: &ContactProps) -> Html {
    let contact = &props.contact;
    let social = &contact.social;
    html! {
        <section id="contact">
            <p class="section-heading">
                <i class="fa-regular fa-address-card" aria-hidden="true"></i>
                { "Contact" }
            </p>
            <div class="contact-links">
                <a class="contact-link" href={social.email.url.clone()}>
                    <i class={social.email.icon.class()} aria-hidden="true"></i>
                    { &contact.email }
                </a>
                { for [&social.github, &social.linkedin, &social.x].into_iter().map(|link| html! {
                    <a class="contact-link" href={link.url.clone()} target="_blank" rel="noopener" aria-label={link.name.clone()}>
                        <i class={link.icon.class()} aria-hidden="true"></i>
                    </a>
                }) }
            </div>
        </section>
    }
}
