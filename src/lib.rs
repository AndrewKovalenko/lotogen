#[macro_use]
extern crate crossterm;

pub mod application {
    mod generator;
    mod lotteries;
    mod menu;
    mod tuimenu;
    mod view;

    mod ui_components {
        pub mod screen;
    }

    use std::io;

    use crossterm::{
        event::{read, DisableMouseCapture},
        terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use generator::LotteryTicket;
    use tui::{
        backend::CrosstermBackend,
        layout::Rect,
        text::{Span, Spans},
        widgets::Paragraph,
        Terminal,
    };

    use self::ui_components::screen;

    pub fn run() {
        let mut screen = screen::Screen::new().unwrap();
        let render_result = screen.show(&|frame| {
            let text = vec![Spans::from(vec![Span::raw("First"), Span::raw(".")])];
            let paragraph = Paragraph::new(text);
            frame.render_widget(paragraph, frame.size());
        });

        read().unwrap();
        screen.restore();
    }
}
