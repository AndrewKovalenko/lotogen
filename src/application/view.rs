use super::ui_components::screen::Screen;
use super::LotteryTicket;

use crossterm::cursor;
use crossterm::event::read;
use crossterm::{
    execute,
    style::{Attribute, Print, SetAttribute, SetForegroundColor},
};
use std::io::Stdout;
use tui::{backend::CrosstermBackend, Terminal};

const NUMBER_OF_GAMES: u8 = 5;
const ONE_GAME_HEIGHT: u8 = 9;
const TICKET_WIDTH: u8 = 65;

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

pub fn show_ticket<'a>(lottery_ticket: &'a Vec<Vec<LotteryTicket>>, number_of_tickets: usize) {
    let mut screen = Screen::new().unwrap();

    screen.show(&|terminal| {
        for ticket_number in 1..=number_of_tickets {
            let left_corner_position = ((ticket_number - 1) as u8) * TICKET_WIDTH;

            for game in 0..NUMBER_OF_GAMES {
                let vertical_offset = game * ONE_GAME_HEIGHT;
                print_field(
                    &lottery_ticket[ticket_number - 1][game as usize].main_field,
                    terminal,
                    3 + vertical_offset as u16,
                    12 + left_corner_position as u16,
                );
                print_field(
                    &lottery_ticket[ticket_number - 1][game as usize].separate_number,
                    terminal,
                    8 + vertical_offset as u16,
                    12 + left_corner_position as u16,
                );
            }
        }

        read().unwrap();
    });
}
