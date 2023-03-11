use super::ui_components::screen::Screen;
use super::LotteryTicket;

use crossterm::event::read;
use tui::style::Style;
use tui::widgets::{Block as TuiBlock, Cell, Row, Table};

const NUMBER_OF_GAMES: u8 = 5;
// const ONE_GAME_HEIGHT: u8 = 9;
// const TICKET_WIDTH: u8 = 65;

fn print_field(nums: &Vec<i8>) {
    let cells = nums.iter().fold(Vec::<Vec<Cell>>::new(), |mut rows, n| {
        let style = if *n < 0 {
            Style::default().fg(tui::style::Color::Green)
        } else {
            Style::default().fg(tui::style::Color::Gray)
        };

        let cell = Cell::from(n.abs().to_string()).style(style);

        if rows.len() == 0 || n % 20 == 0 {
            let new_row = vec![cell];
            rows.push(new_row);
        } else {
            let current_row = rows.last_mut().unwrap();
            current_row.push(cell);
        }

        return rows;
    });

    let table_rows: Vec<Row> = cells
        .iter()
        .map(|row_vector| Row::new(row_vector.clone()))
        .collect();

    Table::new(table_rows)
        .column_spacing(1)
        .block(TuiBlock::default());
}

pub fn show_ticket<'a>(lottery_ticket: &'a LotteryTicket, number_of_tickets: u8) {
    let mut screen = Screen::new().unwrap();

    screen.show(&|_| {
        for _ in 1..=number_of_tickets {
            for _ in 0..NUMBER_OF_GAMES {
                print_field(&lottery_ticket.main_field);
                print_field(&lottery_ticket.separate_number);
            }
            // });
        }

        read().unwrap();
    });
}
