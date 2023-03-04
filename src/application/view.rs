use super::ui_components::block::Block;
use super::ui_components::screen::Screen;
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
use tui::layout::Rect;
use tui::{
    backend::CrosstermBackend,
    widgets::{Block as TuiBlock, Borders},
    Terminal,
};

const NUMBER_OF_GAMES: u8 = 5;
const ONE_GAME_HEIGHT: u8 = 9;

fn print_field(
    nums: &Vec<i8>,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    start_row: u16,
    start_column: u16,
) {
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
            cursor::MoveTo(start_column + (3 * column) as u16, start_row + row),
            SetForegroundColor(color),
            SetAttribute(attr),
            Print(n.abs()),
        )
        .unwrap();

        if n % 20 == 0 {
            row = row + 1;
            column = 0;
        } else {
            column = column + 1;
        }
    });
}

pub fn oop_show(lottery_ticket: &LotteryTicket) {
    let mut screen = Screen::new().unwrap();

    screen.show(&|terminal| {
        let mut ticket = Block::new(terminal);
        let ticket_dimensions = Rect::new(9, 1, 65, (NUMBER_OF_GAMES * ONE_GAME_HEIGHT + 2) as u16);
        ticket.show_block(ticket_dimensions, &|terminal| {
            for game in 0..NUMBER_OF_GAMES {
                let vertical_offset = game * ONE_GAME_HEIGHT;
                print_field(
                    &lottery_ticket.main_field,
                    terminal,
                    3 + vertical_offset as u16,
                    12,
                );
                print_field(
                    &lottery_ticket.separate_number,
                    terminal,
                    8 + vertical_offset as u16,
                    12,
                );
            }
        });

        read().unwrap();
    });
}
