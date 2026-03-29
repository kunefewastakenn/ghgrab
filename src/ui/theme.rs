use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::OnceLock;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ThemeFile {
    bg_color:       Option<[u8; 3]>,
    fg_color:       Option<[u8; 3]>,
    accent_color:   Option<[u8; 3]>,
    warning_color:  Option<[u8; 3]>,
    error_color:    Option<[u8; 3]>,
    success_color:  Option<[u8; 3]>,
    folder_color:   Option<[u8; 3]>,
    selected_color: Option<[u8; 3]>,
    border_color:   Option<[u8; 3]>,
    highlight_bg:   Option<[u8; 3]>,
}

struct Theme {
    pub bg_color:       Color,
    pub fg_color:       Color,
    pub accent_color:   Color,
    pub warning_color:  Color,
    pub error_color:    Color,
    pub success_color:  Color,
    pub folder_color:   Color,
    pub selected_color: Color,
    pub border_color:   Color,
    pub highlight_bg:   Color,
}

fn rgb(val: Option<[u8; 3]>, default: [u8; 3]) -> Color {
    let [r, g, b] = val.unwrap_or(default);
    Color::Rgb(r, g, b)
}

fn load_theme() -> Theme {
    let file: Option<ThemeFile> = (|| -> Option<ThemeFile> {
        let mut path = dirs::config_dir()?;
        path.push("ghgrab");
        path.push("theme.toml");
        let content = fs::read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    })();

    let f = file.as_ref();
    Theme {
        bg_color:       rgb(f.and_then(|t| t.bg_color),       [36, 40, 59]),
        fg_color:       rgb(f.and_then(|t| t.fg_color),       [192, 202, 245]),
        accent_color:   rgb(f.and_then(|t| t.accent_color),   [122, 162, 247]),
        warning_color:  rgb(f.and_then(|t| t.warning_color),  [224, 175, 104]),
        error_color:    rgb(f.and_then(|t| t.error_color),    [247, 120, 107]),
        success_color:  rgb(f.and_then(|t| t.success_color),  [158, 206, 106]),
        folder_color:   rgb(f.and_then(|t| t.folder_color),   [130, 170, 255]),
        selected_color: rgb(f.and_then(|t| t.selected_color), [255, 158, 100]),
        border_color:   rgb(f.and_then(|t| t.border_color),   [86, 95, 137]),
        highlight_bg:   rgb(f.and_then(|t| t.highlight_bg),   [41, 46, 66]),
    }
}

static THEME: OnceLock<Theme> = OnceLock::new();
fn t() -> &'static Theme { THEME.get_or_init(load_theme) }

pub fn BG_COLOR()       -> Color { t().bg_color }
pub fn FG_COLOR()       -> Color { t().fg_color }
pub fn ACCENT_COLOR()   -> Color { t().accent_color }
pub fn WARNING_COLOR()  -> Color { t().warning_color }
pub fn ERROR_COLOR()    -> Color { t().error_color }
pub fn SUCCESS_COLOR()  -> Color { t().success_color }
pub fn FOLDER_COLOR()   -> Color { t().folder_color }
pub fn _SELECTED_COLOR()-> Color { t().selected_color }
pub fn BORDER_COLOR()   -> Color { t().border_color }
pub fn HIGHLIGHT_BG()   -> Color { t().highlight_bg }
