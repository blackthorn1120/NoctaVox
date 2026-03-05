use serde::Deserialize;

#[derive(Deserialize)]
pub struct ExtraScheme {
    pub is_dark: Option<bool>,
    pub decorator: Option<String>,
}
