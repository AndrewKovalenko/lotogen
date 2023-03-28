use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Stdout};
use tui::{backend::CrosstermBackend, Frame, Terminal};

type TerminalFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub struct Screen {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Screen {
    pub fn new() -> Result<Self, std::io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        return Ok(Self { terminal });
    }

    pub fn show(&mut self, render: &dyn Fn(&mut TerminalFrame)) -> Result<(), std::io::Error> {
        self.terminal.draw(|frame| {
            render(frame);
        })?;

        Ok(())
    }

    pub fn restore(&mut self) {
        disable_raw_mode().unwrap();
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();
        self.terminal.show_cursor().unwrap();
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();
        self.terminal.show_cursor().unwrap();
    }
}
