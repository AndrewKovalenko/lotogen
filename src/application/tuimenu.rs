use std::{default, io::Stdout};

use super::lotteries::Lottery;
use super::menu::MenuEvent;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    Frame,
};

enum MenuItem<'a> {
    Lottery(Lottery<'a>),
    Action(&'a str),
}

pub struct Menu<'a> {
    frame: &'a Frame<'a, CrosstermBackend<Stdout>>,
    currently_selected_item: u8,
}

impl<'a> Menu<'a> {
    pub fn new(frame: &'a Frame<'a, CrosstermBackend<Stdout>>) -> Menu<'a> {
        return Menu {
            frame,
            currently_selected_item: 0,
        };
    }

    fn show_menu(&self) {}

    pub fn select(&self) -> MenuEvent {
        let chunks = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(100)])
            .split(self.frame.size());

        loop {
            let items = [
                ListItem::new("Item 1"),
                ListItem::new("Item 2"),
                ListItem::new("Item 3"),
            ];
            List::new(items)
                .block(Block::default().title("List").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>");
        }
    }
}
