use std::cmp::{max, min};

use crossterm::event::{Event, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Rect,
    style::Stylize,
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::app::{App, Mode};

use super::{create_block, Widget};

pub struct ErrorPopup {
    pub error: String,
}

impl ErrorPopup {
    pub fn with_error(&mut self, error: String) {
        self.error = error;
    }
}

impl Default for ErrorPopup {
    fn default() -> Self {
        ErrorPopup {
            error: "".to_owned(),
        }
    }
}

impl Widget for ErrorPopup {
    fn draw(&self, f: &mut Frame, app: &App, area: Rect) {
        let lines = self.error.split("\n");
        let max_line = lines.clone().fold(30, |acc, e| max(e.len(), acc)) as u16 + 3;
        let x_len = min(max_line, area.width - 4);

        // Get number of lines including wrapped lines
        let height = lines.fold(0, |acc, e| {
            acc + (e.len() as f32 / (x_len - 2) as f32).ceil() as u16
        }) + 2;
        let center = super::centered_rect(x_len, height, area);
        let clear = super::centered_rect(center.width + 2, center.height, area);
        let p = Paragraph::new(self.error.to_owned())
            .block(
                create_block(app.theme, true)
                    .fg(app.theme.remake)
                    .title("Error"),
            )
            .wrap(Wrap { trim: false });
        super::clear(f, clear, app.theme.bg);
        f.render_widget(p, center);
    }

    fn handle_event(&mut self, app: &mut App, e: &Event) {
        if let Event::Key(KeyEvent {
            kind: KeyEventKind::Press,
            ..
        }) = e
        {
            if app.errors.len() == 0 {
                app.mode = Mode::Normal;
            }
        }
    }

    fn get_help() -> Option<Vec<(&'static str, &'static str)>> {
        None
    }
}