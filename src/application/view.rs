use super::ui_components::block::Block;
use super::ui_components::screen::Screen;
use super::LotteryTicket;

use crossterm::event::read;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::{Block as TuiBlock, Cell, Row, Table};

const NUMBER_OF_GAMES: u8 = 5;
const ONE_GAME_HEIGHT: u8 = 9;
const TICKET_WIDTH: u8 = 65;

fn print_field(nums: &Vec<i8>) {
    let mut row = 0;
    let mut column = 0;
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

    // if n % 20 == 0 {
    //     row = row + 1;
    //     column = 0;
    // } else {
    //     column = column + 1;
    // }
    // Paragraph::new(n.abs().to_string()).block(block);

    // if n % 20 == 0 {
    //     row = row + 1;
    //     column = 0;
    // } else {
    //     column = column + 1;
    // }
}

pub fn show_ticket<'a>(lottery_ticket: &'a LotteryTicket, number_of_tickets: u8) {
    let mut screen = Screen::new().unwrap();

    screen.show(&|frame| {
        for ticket_number in 1..=number_of_tickets {
            let mut ticket = Block::new(frame);
            let left_corner_position = (ticket_number - 1) * TICKET_WIDTH;
            let ticket_dimensions = Rect::new(
                9 + left_corner_position as u16,
                1,
                65,
                (NUMBER_OF_GAMES * ONE_GAME_HEIGHT + 2) as u16,
            );
            let ticket_title = format!(
                " {} Ticket #{} ",
                lottery_ticket.lottery_name, ticket_number
            );

            ticket.show_block_with_title(ticket_dimensions, ticket_title.as_str(), &|frame| {
                for game in 0..NUMBER_OF_GAMES {
                    print_field(&lottery_ticket.main_field);
                    print_field(&lottery_ticket.separate_number);
                }
            });
        }

        read().unwrap();
    });
}
