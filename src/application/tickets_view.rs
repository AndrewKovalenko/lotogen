use super::ui_components::screen::Screen;
use super::LotteryTicket;

use crossterm::event::read;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Cell, Row, Table};

const NUMBERS_PER_ROW: usize = 20;
const COLUMN_SPACING: u16 = 2;

fn add_to_row<'a>(mut rows: Vec<Vec<Cell<'a>>>, (i, n): (usize, &i8)) -> Vec<Vec<Cell<'a>>> {
    let style = if *n < 0 {
        Style::default().fg(tui::style::Color::Green)
    } else {
        Style::default().fg(tui::style::Color::Gray)
    };

    let cell = Cell::from(n.abs().to_string()).style(style);

    if rows.len() == 0 || i % NUMBERS_PER_ROW == 0 {
        let new_row = vec![cell];
        rows.push(new_row);
    } else {
        let current_row = rows.last_mut().unwrap();
        current_row.push(cell);
    }

    return rows;
}

fn to_table<'a>(numbers: &Vec<i8>) -> Table<'a> {
    let cells = numbers
        .iter()
        .enumerate()
        .fold(Vec::<Vec<Cell>>::new(), add_to_row);

    let table_rows: Vec<Row> = cells
        .iter()
        .map(|row_vector| Row::new(row_vector.clone()))
        .collect();

    Table::new(table_rows).block(Block::default())
}

pub fn show_ticket<'a>(lottery_ticket: &'a LotteryTicket) {
    let mut screen = Screen::new().unwrap();

    screen.show(&|frame| {
        let ticket_layout_sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(7),
                    Constraint::Length(2),
                ]
                .as_ref(),
            )
            .split(frame.size());

        let comumn_width: Vec<Constraint> = (0..NUMBERS_PER_ROW)
            .map(|_| Constraint::Length(2))
            .collect();

        let main_field_numbers = to_table(&lottery_ticket.main_field)
            .style(Style::default().fg(Color::White))
            .block(Block::default().title("Primary numbers"))
            .widths(&comumn_width)
            .column_spacing(COLUMN_SPACING);

        frame.render_widget(main_field_numbers, ticket_layout_sections[1]);

        let super_numbers = to_table(&lottery_ticket.separate_number)
            .style(Style::default().fg(Color::White))
            .block(Block::default().title("Special number"))
            .widths(&comumn_width)
            .column_spacing(COLUMN_SPACING);

        frame.render_widget(super_numbers, ticket_layout_sections[2]);
    });
    read().unwrap();
}
