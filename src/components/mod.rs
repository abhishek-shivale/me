use yew::Properties;

pub mod hero;
pub mod navbar;
pub mod button;


#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    foreground: String,
    background: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            foreground: "var(--text-primary)".into(),
            background: "var(--surface)".into(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub theme: Theme,
}
