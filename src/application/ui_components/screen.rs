use crossterm::{
    event::{read, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Stdout};
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    text::{Span, Spans},
    widgets::Paragraph,
    Frame, Terminal,
};

pub struct Screen {
    // terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Screen {
    pub fn new() -> Result<Self, std::io::Error> {
        return Ok(Self {});
    }

    pub fn show(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.draw(|frame| {
            let text = vec![Spans::from(vec![Span::raw("First"), Span::raw(".")])];
            let paragraph = Paragraph::new(text);
            frame.render_widget(paragraph, frame.size());
        })?;
        read().unwrap();

        disable_raw_mode().unwrap();
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();
        terminal.show_cursor().unwrap();

        Ok(())
    }
}

impl Drop for Screen {
    fn drop(&mut self) {}
}
