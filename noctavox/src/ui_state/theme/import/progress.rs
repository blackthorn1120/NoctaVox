use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProgressScheme {
    pub style: Option<String>,
    pub speed: Option<f32>,

    pub bar: Option<ProgressBarScheme>,
    pub waveform: Option<WaveformScheme>,
    pub oscilloscope: Option<OscilloScheme>,
    pub spectrum: Option<SpectrumScheme>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ProgressGradientRaw {
    Single(String),
    Gradient(Vec<String>),
}

#[derive(Deserialize)]
pub struct ProgressBarScheme {
    pub color: Option<ProgressGradientRaw>,
    pub color_unplayed: Option<ProgressGradientRaw>,
    pub symbol_played: Option<String>,
    pub symbol_unplayed: Option<String>,
}

#[derive(Deserialize)]
pub struct SpectrumScheme {
    pub color: Option<ProgressGradientRaw>,
    pub mirror: Option<bool>,
    pub decay: Option<f32>,
}

#[derive(Deserialize)]
pub struct OscilloScheme {
    pub color: Option<ProgressGradientRaw>,
}

#[derive(Deserialize)]
pub struct WaveformScheme {
    pub color: Option<ProgressGradientRaw>,
    pub color_unplayed: Option<ProgressGradientRaw>,
}
