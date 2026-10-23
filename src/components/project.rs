use yew::prelude::*;

use crate::components::button::Button;
use crate::utils::resume::{LinkKind, Project};

#[derive(Properties, PartialEq)]
pub struct ProjectsProps {
    pub projects: Vec<Project>,
    pub more_href: AttrValue,
}

#[component]
pub fn Projects(props: &ProjectsProps) -> Html {
    html! {
        <section id="projects">
            <p class="project-section-title">
                <i class="fa-regular fa-file" aria-hidden="true"></i>
                { "Projects" }
            </p>
            <div class="projects-section">
                { for props.projects.iter().take(4).map(project_card) }
            </div>
            <Button name="View More Projects" class="more-projects-btn" href={props.more_href.clone()} />
        </section>
    }
}

fn project_card(project: &Project) -> Html {
    let video = project.video.as_deref().filter(|v| !v.is_empty());
    let image = project.image.as_deref().filter(|i| !i.is_empty());
    let media = match (video, image) {
        (Some(src), _) => html! {
            <video
                class="project-img"
                src={src.to_owned()}
                autoplay=true
                loop=true
                playsinline=true
                ~muted={true}
                ~defaultMuted={true}
                ~defaultPlaybackRate={2.0}
            ></video>
        },
        (None, Some(src)) => html! {
            <img class="project-img" src={src.to_owned()} alt={project.title.clone()} loading="lazy" />
        },
        (None, None) => html! {},
    };
    html! {
        <div class="project-card">
            <a class="anchor-proj-img" href={project.href.clone()} target="_blank" rel="noopener">
                { media }
                <i class="fa-solid fa-arrow-up-right-from-square project-icon" aria-hidden="true"></i>
            </a>
            <p class="project-title">{ &project.title }</p>
            <p class="project-desc">{ &project.description }</p>
            <div class="project-tags">
                { for project.technologies.iter().map(|t| html! {
                    <Button name={t.clone()} class="tag" />
                }) }
            </div>
            <div class="project-actions">
                { for project.links.iter().map(|link| {
                    let name = match link.kind {
                        LinkKind::Source => "Source Code",
                        LinkKind::Website => "Live Site",
                    };
                    html! { <Button {name} class="project-btn" href={link.href.clone()} /> }
                }) }
            </div>
        </div>
    }
}
