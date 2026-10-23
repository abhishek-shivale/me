use yew::prelude::*;

use crate::components::hero::Hero;
use crate::utils::resume::Resume;

pub fn home(resume: &Resume) -> Html  {
    html!(
        <Hero personal={resume.personal.clone()} contact={resume.contact.clone()} />
    )
}
