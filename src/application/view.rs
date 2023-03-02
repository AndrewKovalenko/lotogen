use super::LotteryTicket;

use crossterm::event::read;
use crossterm::{cursor, terminal};
use crossterm::{
    event::DisableMouseCapture,
    execute,
    style::{Attribute, Print, SetAttribute, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Stdout};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};

fn print_field(nums: &Vec<i8>, terminal: &mut Terminal<CrosstermBackend<Stdout>>, start_row: u16) {
    let mut row = 0;
    let mut column = 0;
    nums.iter().for_each(|n| {
        let (color, attr) = if *n < 0 {
            (crossterm::style::Color::Green, Attribute::Bold)
        } else {
            (crossterm::style::Color::Grey, Attribute::Reset)
        };

        execute!(
            terminal.backend_mut(),
            cursor::MoveTo(20 + (3 * column) as u16, start_row + row),
            SetForegroundColor(color),
            SetAttribute(attr),
            Print(n.abs()),
        )
        .unwrap();

        if n % 10 == 0 {
            row = row + 1;
            column = 0;
        } else {
            column = column + 1;
        }
    });
}

pub fn show_lottery_ticket(lottery_ticket: &LotteryTicket) {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let size = f.size();
            let block = Block::default().title("Block").borders(Borders::ALL);
            f.render_widget(block, size);
        })
        .unwrap();

    print_field(&lottery_ticket.main_field, &mut terminal, 20);
    print_field(&lottery_ticket.separate_number, &mut terminal, 28);

    read().unwrap();

    // restore terminal
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();
}
