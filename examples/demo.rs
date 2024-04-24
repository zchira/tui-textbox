use std::io::stdout;
use crossterm::{terminal::{EnterAlternateScreen, enable_raw_mode, disable_raw_mode, LeaveAlternateScreen}, execute, event::{DisableMouseCapture, self, KeyEventKind, KeyEvent, KeyCode}, ExecutableCommand};
use ratatui::{Terminal, prelude::{CrosstermBackend, Backend, Layout, Direction, Rect}, Frame, widgets::{Block, Borders}};
use ratatui::layout::Constraint;
use tui_textbox::{TextboxState, Textbox};

pub struct App {
    pub textbox_state: TextboxState,
}

impl App {
    pub fn ui(&mut self, f: &mut Frame) {
        let size = f.size();

        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Percentage(100)])
            .split(size);

        let status_block = Block::default().borders(Borders::ALL).title(format!("Textbox demo"));

        f.render_widget(status_block, vertical_chunks[0]);

        let textbox = Textbox::default();
        let textbox_rect = Rect {
            x: 1,
            y: 1,
            width: 15,
            height: 1,
        };
        f.render_stateful_widget(textbox, textbox_rect, &mut self.textbox_state);
    }

    fn handle_events(&mut self, key: KeyEvent) -> std::io::Result<bool> {
        match (key.code, key.modifiers) {
            (KeyCode::Esc, _) => return Ok(true),
            (key_code, key_modifiers) => {
                self.textbox_state.handle_events(key_code, key_modifiers);
            }
        }
        Ok(false)
    }
}

pub fn main() -> std::io::Result<()> {

    let mut app = App {
        textbox_state: Default::default(),
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
