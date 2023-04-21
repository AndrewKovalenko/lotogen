use super::ui_components::screen::Screen;
use super::LotteryTicket;

use crossterm::event::read;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Cell, Row, Table};

const NUMBERS_PER_ROW: usize = 20;

fn nums_to_table<'a>(nums: &Vec<i8>) -> Table<'a> {
    let cells = nums
        .iter()
        .enumerate()
        .fold(Vec::<Vec<Cell>>::new(), |mut rows, (i, n)| {
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
        });

    let table_rows: Vec<Row> = cells
        .iter()
        .map(|row_vector| Row::new(row_vector.clone()))
        .collect();

    Table::new(table_rows)
        .column_spacing(1)
        .block(Block::default())
}

pub fn show_ticket<'a>(lottery_ticket: &'a LotteryTicket) {
    let mut screen = Screen::new().unwrap();

    screen.show(&|frame| {
        let ticket_layout_sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(2)].as_ref())
            .split(frame.size());

        let comumn_width: Vec<Constraint> = (0..20).map(|_| Constraint::Length(2)).collect();

        let table = nums_to_table(&lottery_ticket.main_field)
            // You can set the style of the entire Table.
            .style(Style::default().fg(Color::White))
            .block(Block::default().title("Table"))
            // Columns widths are constrained in the same way as Layout...
            .widths(&comumn_width)
            // ...and they can be separated by a fixed spacing.
            .column_spacing(1);
        frame.render_widget(table, ticket_layout_sections[1]);

        // let main_field_table = nums_to_table(&lottery_ticket.main_field);
        // frame.render_widget(main_field_table, frame.size());
    });
    read().unwrap();
}
