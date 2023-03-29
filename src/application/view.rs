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
        let table = Table::new(vec![
            // Row can be created from simple strings.
            Row::new(vec!["Row11", "Row12", "Row13"]),
            // You can style the entire row.
            Row::new(vec!["Row21", "Row22", "Row23"]).style(Style::default().fg(Color::Blue)),
            // If you need more control over the styling you may need to create Cells directly
            Row::new(vec![
                Cell::from("Row31"),
                Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
                Cell::from(Spans::from(vec![
                    Span::raw("Row"),
                    Span::styled("33", Style::default().fg(Color::Green)),
                ])),
            ]),
            // If a Row need to display some content over multiple lines, you just have to change
            // its height.
            Row::new(vec![
                Cell::from("Row\n41"),
                Cell::from("Row\n42"),
                Cell::from("Row\n43"),
            ])
            .height(2),
        ])
        // You can set the style of the entire Table.
        .style(Style::default().fg(Color::White))
        // It has an optional header, which is simply a Row always visible at the top.
        .header(
            Row::new(vec!["Col1", "Col2", "Col3"])
                .style(Style::default().fg(Color::Yellow))
                // If you want some space between the header and the rest of the rows, you can always
                // specify some margin at the bottom.
                .bottom_margin(1),
        )
        // As any other widget, a Table can be wrapped in a Block.
        .block(Block::default().title("Table"))
        // Columns widths are constrained in the same way as Layout...
        .widths(&[
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(10),
        ])
        // ...and they can be separated by a fixed spacing.
        .column_spacing(1);
        frame.render_widget(table, frame.size());
        read().unwrap();

        // let main_field_table = nums_to_table(&lottery_ticket.main_field);
        // frame.render_widget(main_field_table, *ticket_layout_sections.first().unwrap());
        // read().unwrap();
        // // nums_to_table(
        //     &lottery_ticket.separate_number,
        //     frame,
        //     ticket_layout_sections.last().unwrap(),
        // );
        // read().unwrap();
    });
}
