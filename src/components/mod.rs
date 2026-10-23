use yew::Properties;

pub mod hero;
pub mod button;


#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    foreground: String,
    background: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub theme: Theme,
}
