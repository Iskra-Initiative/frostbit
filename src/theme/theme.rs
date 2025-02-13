use iced::theme;

const DEFAULT_THEME_NAME: &str = "Ferra";
#[derive(Debug, Clone)]
pub struct Ferra {
    pub background: iced::Color,
    pub text: iced::Color,
}

impl Default for Ferra {
    fn default() -> Self {
        Self {
            background: iced::Color::parse("#e06b75").expect("Invalid color"),
            text: iced::Color::parse("#f5d76e").expect("Invalid color"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Themes {
    Ferra(Ferra),
}

#[derive(Debug, Clone)]
pub struct Theme {
    name: String,
    theme: Themes,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: DEFAULT_THEME_NAME.to_string(),
            theme: Themes::Ferra(Ferra::default()),
        }
    }
}
