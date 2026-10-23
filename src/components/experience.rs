use yew::prelude::*;

use crate::utils::resume::{Education, Work};

#[derive(Properties, PartialEq)]
pub struct ExperienceProps {
    pub work: Vec<Work>,
    pub education: Vec<Education>,
}

#[component]
pub fn Experience(props: &ExperienceProps) -> Html {
    html! {
        <>
            <section id="experience">
                <p class="section-heading">
                    <i class="fa-regular fa-clipboard" aria-hidden="true"></i>
                    { "Experience" }
                </p>
                { for props.work.iter().map(|w| entry(&w.logo_url, &w.title, &w.company, &w.start, &w.end)) }
            </section>
            <section id="education">
                <p class="section-heading">
                    <i class="fa-solid fa-graduation-cap" aria-hidden="true"></i>
                    { "Education" }
                </p>
                { for props.education.iter().map(|e| entry(&e.logo_url, &e.degree, &e.school, &e.start, &e.end)) }
            </section>
        </>
    }
}

fn entry(logo: &str, role: &str, org: &str, start: &str, end: &str) -> Html {
    html! {
        <div class="job-card">
            <img class="job-img" src={logo.to_owned()} alt={org.to_owned()} loading="lazy" />
            <div class="job-left">
                <p class="job-role">{ role }</p>
                <p class="job-company-name">{ org }</p>
            </div>
            <p class="job-time">{ format!("{start} - {end}") }</p>
        </div>
    }
}
