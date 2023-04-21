use super::generator::GAMES_PER_TICKET;
use super::ui_components::screen::{Screen, TerminalFrame};
use super::Ticket;

use crossterm::event::read;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Cell, Row, Table};

const NUMBERS_PER_ROW: usize = 20;
const COLUMN_SPACING: u16 = 2;
const MAIN_FIELD_HEIGHT: u16 = 5;
const SPECIAL_FIELD_HEIGHT: u16 = 2;
const TICKET_MARGIN: u16 = 3;

fn add_to_row<'a>(mut rows: Vec<Vec<Cell<'a>>>, (i, n): (usize, &i8)) -> Vec<Vec<Cell<'a>>> {
    let style = if *n < 0 {
        Style::default()
            .fg(tui::style::Color::Green)
            .add_modifier(Modifier::BOLD)
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

fn get_games_layout(number_of_games: usize) -> Layout {
    let game_sections: Vec<Constraint> = (0..number_of_games)
        .map(|_| Constraint::Length(9))
        .collect();

    Layout::default()
        .direction(Direction::Vertical)
        .constraints(game_sections)
}

fn show_ticket<'a>(lottery_ticket: &Ticket, frame: &mut TerminalFrame<'a>, section: &Rect) {
    let games_sections = get_games_layout(GAMES_PER_TICKET).split(*section);

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

        let comumn_width: Vec<Constraint> = (0..NUMBERS_PER_ROW)
            .map(|_| Constraint::Length(2))
            .collect();

        let main_field_numbers = to_table(&lottery_ticket.games[game_number].main_field)
            .style(Style::default().fg(Color::White))
            .widths(&comumn_width)
            .column_spacing(COLUMN_SPACING);

        frame.render_widget(main_field_numbers, game_fields[0]);

        let super_numbers = to_table(&lottery_ticket.games[game_number].separate_number)
            .style(Style::default().fg(Color::White))
            .widths(&comumn_width)
            .column_spacing(COLUMN_SPACING);

        frame.render_widget(super_numbers, game_fields[1]);
    }
}

pub fn show_results<'a>(tickets: &'a Vec<Ticket>) {
    let mut screen = Screen::new().unwrap();
    screen.show(&|frame| {
        let results_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(TICKET_MARGIN)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(40)].as_ref())
            .split(frame.size());

        for ticket_number in 0..tickets.len() {
            show_ticket(
                &tickets[ticket_number],
                frame,
                &results_layout[ticket_number],
            );
        }
    });
    read().unwrap();
}
