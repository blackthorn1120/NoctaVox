use anyhow::Result;
use ratatui::style::Color;

use crate::ui_state::{
    InactiveGradient, ProgressGradient,
    theme::{
        BAR_SYMBOL_PLAYED, BAR_SYMBOL_UNPLAYED,
        import::{OscilloScheme, ProgressBarScheme, SpectrumScheme, WaveformScheme},
    },
};

#[derive(Clone)]
pub struct ParsedBar {
    pub active_color: ProgressGradient,
    pub inactive_color: InactiveGradient,

    pub played_symbol: String,
    pub unplayed_symbol: String,
}

impl ParsedBar {
    pub(crate) fn parse(p: Option<&ProgressBarScheme>, a: Color) -> Result<Self> {
        match p {
            Some(bar) => Ok(ParsedBar {
                active_color: match bar.color.as_ref() {
                    Some(raw) => ProgressGradient::from_raw(raw)?,
                    None => ProgressGradient::Static(a),
                },
                inactive_color: match bar.color_unplayed.as_ref() {
                    Some(raw) => InactiveGradient::from_raw(raw)?,
                    None => InactiveGradient::Dimmed,
                },
                played_symbol: bar
                    .symbol_played
                    .as_deref()
                    .unwrap_or(BAR_SYMBOL_PLAYED)
                    .to_string(),
                unplayed_symbol: bar
                    .symbol_unplayed
                    .as_deref()
                    .unwrap_or(BAR_SYMBOL_UNPLAYED)
                    .to_string(),
            }),
            _ => Ok(ParsedBar {
                active_color: ProgressGradient::Static(a),
                inactive_color: InactiveGradient::Dimmed,
                played_symbol: BAR_SYMBOL_PLAYED.to_string(),
                unplayed_symbol: BAR_SYMBOL_UNPLAYED.to_string(),
            }),
        }
    }
}

#[derive(Clone)]
pub struct ParsedOscilloscope {
    pub color: ProgressGradient,
}

impl ParsedOscilloscope {
    pub(crate) fn parse(p: Option<&OscilloScheme>, a: Color) -> Result<Self> {
        match p {
            Some(oscillo) => Ok(ParsedOscilloscope {
                color: match oscillo.color.as_ref() {
                    Some(raw) => ProgressGradient::from_raw(raw)?,
                    None => ProgressGradient::Static(a),
                },
            }),
            None => Ok(ParsedOscilloscope {
                color: ProgressGradient::Static(a),
            }),
        }
    }
}

#[derive(Clone)]
pub struct ParsedSpectrum {
    pub colors: ProgressGradient,
    pub mirror: bool,
    pub decay: f32,
}

impl ParsedSpectrum {
    pub(crate) fn parse(p: Option<&SpectrumScheme>, a: Color) -> Result<Self> {
        match p {
            Some(spectrum) => Ok(ParsedSpectrum {
                colors: match spectrum.color.as_ref() {
                    Some(raw) => ProgressGradient::from_raw(raw)?,
                    None => ProgressGradient::Static(a),
                },
                mirror: spectrum.mirror.unwrap_or(super::SPECTRUM_MIRROR),
                decay: spectrum
                    .decay
                    .unwrap_or(super::SPECTRUM_DECAY)
                    .clamp(0.7, 0.97),
            }),
            None => Ok(ParsedSpectrum {
                colors: ProgressGradient::Static(a),
                mirror: super::SPECTRUM_MIRROR,
                decay: super::SPECTRUM_DECAY,
            }),
        }
    }
}

#[derive(Clone)]
pub struct ParsedWaveform {
    pub active_color: ProgressGradient,
    pub inactive_color: InactiveGradient,
}

impl ParsedWaveform {
    pub(crate) fn parse(p: Option<&WaveformScheme>, a: Color) -> Result<Self> {
        match p {
            Some(wf) => Ok(ParsedWaveform {
                active_color: match wf.color.as_ref() {
                    Some(raw) => ProgressGradient::from_raw(raw)?,
                    None => ProgressGradient::Static(a),
                },
                inactive_color: match wf.color_unplayed.as_ref() {
                    Some(raw) => InactiveGradient::from_raw(raw)?,
                    None => InactiveGradient::Dimmed,
                },
            }),
            None => Ok(ParsedWaveform {
                active_color: ProgressGradient::Static(a),
                inactive_color: InactiveGradient::Dimmed,
            }),
        }
    }
}
