use yew::Properties;

pub mod hero;
pub mod navbar;
pub mod project;
pub mod experience;
pub mod skills;
pub mod contact;
pub mod footer;
pub mod button;


#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    foreground: String,
    background: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            foreground: "var(--text-muted)".into(),
            background: "var(--surface-raised)".into(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub theme: Theme,
}
