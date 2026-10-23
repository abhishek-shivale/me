use yew::prelude::*;

use crate::components::button::Button;

#[derive(Properties, PartialEq)]
pub struct SkillsProps {
    pub skills: Vec<String>,
}

#[component]
pub fn Skills(props: &SkillsProps) -> Html {
    html! {
        <section id="technologies">
            <p class="section-heading">
                <i class="fa-solid fa-code" aria-hidden="true"></i>
                { "Tech Stack" }
            </p>
            <div class="project-tags">
                { for props.skills.iter().map(|skill| html! {
                    <Button name={skill.clone()} class="tag" />
                }) }
            </div>
        </section>
    }
}
