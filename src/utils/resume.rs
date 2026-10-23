use serde::{Deserialize, Serialize};
use std::error::Error;

// file path to resume.toml
pub const FILE_PATH: &str = "Resume.toml";

// added for browser to work
const RESUME_TOML: &str = include_str!("../../Resume.toml");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resume {
    pub personal: Personal,
    pub navbar: Vec<NavItem>,
    pub contact: Contact,
    pub work: Vec<Work>,
    pub education: Vec<Education>,
    pub projects: Vec<Project>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Personal {
    pub name: String,
    pub initials: String,
    pub url: String,
    pub location: String,
    pub location_link: String,
    pub description: String,
    pub title: String,
    pub bio: Vec<String>,
    pub summary: String,
    pub avatar_url: String,
    pub skills: Vec<String>,
}

impl Personal {
    pub fn description_with_yoe(&self, yoe: u32) -> String {
        self.description.replace("{{YOE}}", &yoe.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavItem {
    pub href: String,
    pub icon: Icon,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub email: String,
    pub social: Social,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Social {
    #[serde(rename = "GitHub")]
    pub github: SocialLink,
    #[serde(rename = "LinkedIn")]
    pub linkedin: SocialLink,
    #[serde(rename = "X")]
    pub x: SocialLink,
    pub email: SocialLink,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialLink {
    pub name: String,
    pub url: String,
    pub icon: Icon,
    pub navbar: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Work {
    pub company: String,
    pub href: String,
    #[serde(default)]
    pub badges: Vec<String>,
    pub location: String,
    pub title: String,
    pub logo_url: String,
    pub start: String,
    pub end: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Education {
    pub school: String,
    pub href: String,
    pub degree: String,
    pub logo_url: String,
    pub start: String,
    pub end: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub href: String,
    pub dates: String,
    pub active: bool,
    pub description: String,
    pub technologies: Vec<String>,
    pub links: Vec<ProjectLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectLink {
    #[serde(rename = "type")]
    pub kind: LinkKind,
    pub href: String,
    pub icon: Icon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkKind {
    Source,
    Website,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Icon {
    Home,
    Notebook,
    Github,
    Linkedin,
    X,
    Email,
    Globe,
    Youtube,
}

impl Resume {
    pub fn parse() -> Result<Self, Box<dyn Error>> {
        let mut resume: Self = toml::from_str(RESUME_TOML)
            .map_err(|e| format!("failed to parse {FILE_PATH}: {e}"))?;

        resume.personal.description = resume.personal.description_with_yoe(2);

        Ok(resume)
    }
}
