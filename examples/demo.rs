use std::io::stdout;
use crossterm::{terminal::{EnterAlternateScreen, enable_raw_mode, disable_raw_mode, LeaveAlternateScreen}, execute, event::{DisableMouseCapture, self, KeyEventKind, KeyEvent, KeyCode}, ExecutableCommand};
use ratatui::{Terminal, prelude::{CrosstermBackend, Backend, Layout, Direction, Rect}, Frame, widgets::{Block, Borders}};
use ratatui::layout::Constraint;
use tui_textbox::{TextboxState, Textbox};

pub struct App {
    pub textbox_state: TextboxState,
    pub textbox2_state: TextboxState,
    pub focused_textbox: u8
}

impl App {
    pub fn ui(&mut self, f: &mut Frame) {
        let size = f.area();

        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Percentage(100)])
            .split(size);

        let block1 = Block::default().borders(Borders::ALL).title(format!("Use <TAB> to switch active textbox, <ESC> to exit demo"));
        f.render_widget(block1, vertical_chunks[0]);

        let textbox = Textbox::default()
            .render_cursor(self.focused_textbox == 0);
        let textbox_rect = Rect {
            x: 1,
            y: 1,
            width: 15,
            height: 1,
        };
        f.render_stateful_widget(textbox, textbox_rect, &mut self.textbox_state);

        let block2 = Block::default().borders(Borders::ALL);
        f.render_widget(block2, vertical_chunks[1]);

        let textbox2 = Textbox::default()
            .fg(ratatui::style::Color::Yellow)
            .bg(ratatui::style::Color::Green)
            .hint_color(ratatui::style::Color::LightGreen)
            .cursor_color(ratatui::style::Color::DarkGray)
            .render_cursor(self.focused_textbox == 1);
        f.render_stateful_widget(textbox2, vertical_chunks[1].inner(ratatui::layout::Margin::new(1, 1)), &mut self.textbox2_state);
    }

    fn handle_events(&mut self, key: KeyEvent) -> std::io::Result<bool> {
        match (key.code, key.modifiers) {
            (KeyCode::Esc, _) => return Ok(true),
            (KeyCode::Tab, _) => { self.focused_textbox = (self.focused_textbox + 1) % 2;  },
            (key_code, key_modifiers) => {
                match self.focused_textbox {
                    0 => self.textbox_state.handle_events(key_code, key_modifiers),
                    1 => self.textbox2_state.handle_events(key_code, key_modifiers),
                    _ => {}
                };
            }
        }
        Ok(false)
    }
}

pub fn main() -> std::io::Result<()> {

    let mut app = App {
        textbox_state: Default::default(),
        textbox2_state: Default::default(),
        focused_textbox: 0
    };

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    let _res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
        )?;
    terminal.show_cursor()?;

    Ok(())
}


pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> std::io::Result<bool> {
    loop {
        terminal.draw(|f| app.ui(f))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if let Ok(true) = app.handle_events(key) {
                        return Ok(true);
                    }
                }
            }
        }
    }
}
