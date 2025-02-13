/* application window */
pub const WINDOW_WIDTH: f32 = 640.0;
pub const WINDOW_HEIGHT: f32 = 480.0;
pub const WINDOW_TITLE: &str = "frostbit";
pub const WINDOW_POSITION: iced::window::Position = iced::window::Position::Centered;
pub const WINDOW_MIN_SIZE: Option<iced::Size> = None;
pub const WINDOW_MAX_SIZE: Option<iced::Size> = None;
pub const WINDOW_VISIBLE: bool = true;
pub const WINDOW_RESIZABLE: bool = true;
pub const WINDOW_DECORATIONS: bool = true;
pub const WINDOW_TRANSPARENT: bool = false;
pub const WINDOW_LEVEL: iced::window::Level = iced::window::Level::Normal;
pub const WINDOW_ICON: Option<iced::window::Icon> = None;
pub const WINDOW_EXIT_ON_CLOSE_REQUEST: bool = true;

pub const WINDOW_PLATFORM_SPECIFIC: iced::window::settings::PlatformSpecific =
    iced::window::settings::PlatformSpecific {
        drag_and_drop: true,
        skip_taskbar: false,
        undecorated_shadow: false,
    };

pub const APP_SETTINGS: iced::Settings = iced::Settings {
    id: None,
    fonts: Vec::new(),
    default_font: iced::Font::DEFAULT,
    default_text_size: iced::Pixels(16.0),
    antialiasing: true,
};

pub const WINDOW_SETTINGS: iced::window::Settings = iced::window::Settings {
    size: iced::Size::new(WINDOW_WIDTH, WINDOW_HEIGHT),
    position: WINDOW_POSITION,
    min_size: WINDOW_MIN_SIZE,
    max_size: WINDOW_MAX_SIZE,
    visible: WINDOW_VISIBLE,
    resizable: WINDOW_RESIZABLE,
    decorations: WINDOW_DECORATIONS,
    transparent: WINDOW_TRANSPARENT,
    level: WINDOW_LEVEL,
    icon: WINDOW_ICON,
    exit_on_close_request: WINDOW_EXIT_ON_CLOSE_REQUEST,
    platform_specific: WINDOW_PLATFORM_SPECIFIC,
};
