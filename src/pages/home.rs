use yew::prelude::*;

use crate::components::hero::Hero;
use crate::components::contact::ContactSection;
use crate::components::experience::Experience;
use crate::components::project::Projects;
use crate::components::skills::Skills;
use crate::utils::resume::Resume;

pub fn home(resume: &Resume) -> Html  {
    html!(
        <>
            <Hero personal={resume.personal.clone()} contact={resume.contact.clone()} />
            <Projects
                projects={resume.projects.clone()}
                more_href={resume.contact.social.github.url.clone()}
            />
            <Experience work={resume.work.clone()} education={resume.education.clone()} />
            <Skills skills={resume.personal.skills.clone()} />
            <ContactSection contact={resume.contact.clone()} />
        </>
    )
}
