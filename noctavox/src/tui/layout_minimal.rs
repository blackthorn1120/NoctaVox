use crate::ui_state::{Mode, ProgressDisplay, UiState};
use ratatui::layout::{Constraint, Layout, Rect};

pub struct LayoutMinimal {
    pub search_bar: Rect,
    pub content: Rect,
    pub widget: Rect,
}

impl LayoutMinimal {
    pub fn new(area: Rect, state: &mut UiState) -> Self {
        let is_progress_display = state.is_progress_display();

        let search_height = match state.get_mode() == Mode::Search {
            true => match state.borders_enabled() {
                true => 5,
                false => 3,
            },
            false => 0,
        };

        let widget_h = match is_progress_display {
            false => 0,
            true => match (state.get_progress_display(), area.height > 20) {
                (ProgressDisplay::ProgressBar, _) | (_, false) => 3,
                _ => (area.height as f32 * 0.15).ceil() as u16,
            },
        };

        let [_lpadding, main_area, _rpadding] = Layout::horizontal([
            Constraint::Percentage(20),
            Constraint::Fill(1),
            Constraint::Percentage(20),
        ])
        .areas(area);

        let [_upper_pad, upper_block, widget, _bottom_pad] = Layout::vertical([
            Constraint::Percentage(20),
            Constraint::Fill(1),
            Constraint::Length(widget_h),
            Constraint::Percentage(match is_progress_display {
                true => 15,
                false => 20,
            }),
        ])
        .areas(main_area);

        let [search_bar, song_window] =
            Layout::vertical([Constraint::Length(search_height), Constraint::Fill(100)])
                .areas(upper_block);

        LayoutMinimal {
            search_bar,
            content: song_window,
            widget,
        }
    }
}
