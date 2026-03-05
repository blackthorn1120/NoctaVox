mod borders;
mod colors;
mod extras;
mod progress;

pub use progress::*;

#[derive(serde::Deserialize)]
pub struct ThemeImport {
    pub colors: colors::ColorScheme,
    pub borders: Option<borders::BorderScheme>,
    pub progress: Option<progress::ProgressScheme>,
    pub extras: Option<extras::ExtraScheme>,
}
