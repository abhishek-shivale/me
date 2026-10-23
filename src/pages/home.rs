use yew::prelude::*;

use crate::components::hero::Hero;
use crate::components::project::Projects;
use crate::utils::resume::Resume;

pub fn home(resume: &Resume) -> Html  {
    html!(
        <>
            <Hero personal={resume.personal.clone()} contact={resume.contact.clone()} />
            <Projects
                projects={resume.projects.clone()}
                more_href={resume.contact.social.github.url.clone()}
            />
        </>
    )
}
