use crate::ui_state::{LibraryView, Mode, Pane, UiState, fade_color};
use ratatui::{
    style::Stylize,
    text::{Line, Span},
    widgets::{StatefulWidget, Widget},
};

pub struct BreadCrumbs;

impl StatefulWidget for BreadCrumbs {
    type State = UiState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        if !matches!(state.get_mode(), Mode::Library(_)) {
            return;
        }

        let theme = &state.theme_manager.get_display_theme(true);
        let top_level = state.get_sidebar_view();
        let sidebar = top_level.to_string();
        let album_sort = state.get_album_sort_string();

        let bc_highlight = fade_color(theme.dark, theme.accent, 0.85);

        let n = area
            .width
            .saturating_sub(sidebar.len() as u16)
            .saturating_sub(album_sort.len() as u16) as usize;

        let spans = match state.get_pane() {
            Pane::SideBar => {
                vec![
                    Span::from(sidebar).fg(bc_highlight),
                    Span::from(" ".repeat(n)),
                    Span::from(format!("  {}", album_sort)).fg(fade_color(
                        theme.dark,
                        theme.text_muted,
                        0.75,
                    )),
                ]
            }
            Pane::TrackList => match top_level {
                LibraryView::Albums => {
                    let Some(album) = state.get_selected_album() else {
                        return;
                    };
                    Vec::from([
                        Span::from(format!("{top_level}  ")).fg(theme.text_muted),
                        Span::from(format!("{}", album.title)).fg(bc_highlight),
                        Span::from(format!(" [{}]", album.artist)).fg(theme.text_muted),
                    ])
                }
                LibraryView::Playlists => {
                    let Some(playlist) = state.get_selected_playlist() else {
                        return;
                    };
                    Vec::from([
                        Span::from(format!("{top_level}  ")).fg(theme.text_muted),
                        Span::from(format!("{}", playlist.name)).fg(bc_highlight),
                    ])
                }
            },
            _ => vec![],
        };

        Line::from(spans).render(area, buf);
    }
}
