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
        let render_result = screen::Screen::new().unwrap().show();
    }
}
