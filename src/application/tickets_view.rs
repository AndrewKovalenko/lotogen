use super::generator::GAMES_PER_TICKET;
use super::settings::get_lottery_settings;
use super::ui_components::screen::{Screen, TerminalFrame};
use super::Ticket;

use crossterm::event::read;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Cell, Row, Table};

const NUMBERS_PER_ROW: usize = 16;
const COLUMN_SPACING: u16 = 2;
const MAIN_FIELD_HEIGHT: u16 = 6;
const SPECIAL_FIELD_HEIGHT: u16 = 2;
const BLANK_CELL_NUMBER: i8 = -127;
const TICKET_WIDTH: u16 = 62;
const ONE: usize = 1;

fn add_to_row<'a>(mut rows: Vec<Vec<Cell<'a>>>, (i, n): (usize, &i8)) -> Vec<Vec<Cell<'a>>> {
    let style = if *n < 0 {
        Style::default()
            .fg(tui::style::Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(tui::style::Color::DarkGray)
    };

    let cell = if *n == BLANK_CELL_NUMBER {
        Cell::from("")
    } else {
        Cell::from(n.abs().to_string()).style(style)
    };

    if rows.len() == 0 || i % NUMBERS_PER_ROW == 0 {
        let new_row = vec![cell];
        rows.push(new_row);
    } else {
        let current_row = rows.last_mut().unwrap();
        current_row.push(cell);
    }

    return rows;
}

fn to_table<'a>(numbers: &Vec<i8>, offset: usize, offset_rows: usize) -> Table<'a> {
    let formatting_cells = vec![BLANK_CELL_NUMBER; offset];
    let items_per_offset_row = NUMBERS_PER_ROW - offset;
    let mut cell_values = Vec::new();

    for row in 0..offset_rows {
        let row_numbers: &[i8] =
            &numbers[row * items_per_offset_row..row * items_per_offset_row + items_per_offset_row];
        cell_values.extend(formatting_cells.clone());
        cell_values.extend(row_numbers);
    }

    let numbers_left = &numbers[offset_rows * items_per_offset_row..];
    cell_values.extend(numbers_left);

    let cells = cell_values
        .iter()
        .enumerate()
        .fold(Vec::<Vec<Cell>>::new(), add_to_row);

    let table_rows: Vec<Row> = cells
        .iter()
        .map(|row_vector| Row::new(row_vector.clone()))
        .collect();

    Table::new(table_rows)
}

fn get_games_layout(number_of_games: usize) -> Layout {
    let game_sections: Vec<Constraint> = (0..number_of_games)
        .map(|_| Constraint::Length(10))
        .collect();

    Layout::default()
        .direction(Direction::Vertical)
        .constraints(game_sections)
}

fn show_ticket<'a>(lottery_ticket: &Ticket, frame: &mut TerminalFrame<'a>, section: &Rect) {
    let games_sections = get_games_layout(GAMES_PER_TICKET).split(*section);
    let settings = get_lottery_settings(&lottery_ticket.lottery);

    for game_number in 0..GAMES_PER_TICKET {
        let current_game_section = games_sections[game_number];
        let game_fields = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(MAIN_FIELD_HEIGHT),
                    Constraint::Length(SPECIAL_FIELD_HEIGHT),
                ]
                .as_ref(),
            )
            .split(current_game_section);

        let column_width: Vec<Constraint> = (0..NUMBERS_PER_ROW)
            .map(|_| Constraint::Length(2))
            .collect();

        let main_field_numbers = to_table(
            &lottery_ticket.games[game_number].main_field,
            settings.main_field_offset,
            settings.main_field_offset_rows,
        )
        .style(Style::default().fg(Color::White))
        .widths(&column_width)
        .column_spacing(COLUMN_SPACING);

        frame.render_widget(main_field_numbers, game_fields[0]);

        let underline = if game_number != GAMES_PER_TICKET - 1 {
            Block::default().borders(Borders::BOTTOM)
        } else {
            Block::default()
        };

        let super_numbers = to_table(
            &lottery_ticket.games[game_number].separate_number,
            settings.separate_number_offset,
            ONE,
        )
        .style(Style::default().fg(Color::White))
        .widths(&column_width)
        .column_spacing(COLUMN_SPACING)
        .block(underline);

        frame.render_widget(super_numbers, game_fields[1]);
    }
}

pub fn show_results<'a>(tickets: &'a Vec<Ticket>) {
    let mut screen = Screen::new().unwrap();
    screen.show(&|frame| {
        let results_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(15),
                    Constraint::Length(TICKET_WIDTH),
                    Constraint::Percentage(10),
                    Constraint::Length(TICKET_WIDTH),
                    Constraint::Percentage(15),
                ]
                .as_ref(),
            )
            .split(frame.size());

        for ticket_number in 0..tickets.len() {
            show_ticket(
                &tickets[ticket_number],
                frame,
                &results_layout[ticket_number * 2 + 1],
            );
        }
    });
    read().unwrap();
}
