use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{widgets::StatefulWidget, style::{Style, Modifier}};

pub struct Textbox {
    bg_style: Style,
    text_style: Style,
    hint_text_style: Style,
    cursor_style: Style,
}

impl Default for Textbox {
    fn default() -> Self {
        Self {
            bg_style: Style::default().bg(ratatui::style::Color::LightBlue),
            text_style: Style::default().bg(ratatui::style::Color::LightBlue).fg(ratatui::style::Color::White),
            hint_text_style: Style::default().add_modifier(Modifier::ITALIC).bg(ratatui::style::Color::LightBlue).fg(ratatui::style::Color::Gray),
            cursor_style: Style::default().bg(ratatui::style::Color::LightRed),
        }
    }
}

pub struct TextboxState {
    pub cursor_pos: usize,
    pub text: String,
    pub hint_text: Option<String>,
    start: usize
}

impl Default for TextboxState {
    fn default() -> Self {
        Self {
            cursor_pos: Default::default(),
            text: Default::default(),
            hint_text: Some("<hint text>".to_string()),
            start: 0
        }
    }
}

impl TextboxState {
    pub fn handle_events(&mut self, key_code: KeyCode, key_modifiers: KeyModifiers) {
        match (key_code, key_modifiers) {
            (KeyCode::Left, _) => {
                self.cursor_pos = if self.cursor_pos > 0 { self.cursor_pos - 1 } else { self.cursor_pos };
            },
            (KeyCode::Right, _) => {
                self.cursor_pos = if self.cursor_pos < self.text.len() { self.cursor_pos + 1 } else { self.text.len() };
            },
            (KeyCode::Backspace, _) => {
                if self.cursor_pos > 0 {
                    self.cursor_pos = std::cmp::max(self.cursor_pos - 1, 0);
                    self.text.remove(self.cursor_pos);
                }
            },
            (KeyCode::Delete, _) => {
                if self.cursor_pos < self.text.len() {
                    self.text.remove(self.cursor_pos);

                    if self.cursor_pos == self.text.len() && self.text.len() > 0 {
                        self.cursor_pos = self.cursor_pos - 1;
                    }
                }
            },
            (KeyCode::Char(x), _) => {
                self.text.insert(self.cursor_pos, x);
                self.cursor_pos = self.cursor_pos + 1;
            },
            (_, _) => {}
        }
    }
}

impl StatefulWidget for Textbox {
    type State = TextboxState;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        buf.set_style(area, self.bg_style);
        if state.text.len() > 0 {
            let w = usize::from(area.width) - 1;
            if state.cursor_pos > state.start + w  {
               state.start = state.cursor_pos - w;
            };

            if state.cursor_pos < state.start {
                state.start = state.cursor_pos;
            }

            let end = std::cmp::min(state.start + w + 1, state.text.len());

            let visible_text = &state.text[state.start..end];
            buf.set_string(area.x, area.y, visible_text, self.text_style);
        } else {
            if let Some(hint) = state.hint_text.as_ref() {
                buf.set_string(area.x, area.y, hint.clone(), self.hint_text_style );
            }

        }
        let pos_char = state.text.chars().nth(state.cursor_pos).unwrap_or(' ');
        let cur_pos = u16::try_from(state.cursor_pos.checked_sub(state.start).unwrap_or(0)).unwrap_or(0);

        buf.set_string(area.x + cur_pos, area.y, format!("{}", &pos_char), self.cursor_style);
    }
}
