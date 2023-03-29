use std::io::Stdout;

use super::lotteries::Lottery;
use super::ui_components::screen::Screen;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Widget};
use tui::Frame;

pub enum MenuEvent<'a> {
    MenuItemSelected(Lottery<'a>),
    Shutdown,
}

const MIDDLE_BLOCK_INDEX: usize = 1;
const SUPPORTED_LOTTERIES: [Lottery; 2] = [
    Lottery::PowerBall("Power Ball"),
    Lottery::MegaMillions("Mega Millions"),
];
const EXIT_CHAR: char = 'q';

fn lotteries_to_menu_items<'a>(lotteries: &'a [Lottery]) -> Vec<ListItem<'a>> {
    lotteries
        .iter()
        .map(|l| {
            let lottery_name = match *l {
                Lottery::MegaMillions(name) => name,
                Lottery::PowerBall(name) => name,
            };

            ListItem::new(lottery_name)
        })
        .collect()
}

fn get_footer<'a>() -> Paragraph<'a> {
    let text = vec![Spans::from(vec![
        Span::raw("Press"),
        Span::styled(" 'q' ", Style::default().add_modifier(Modifier::ITALIC)),
        Span::raw("key to quit"),
    ])];
    Paragraph::new(text).alignment(Alignment::Center)
}

fn get_layout(frame: &Frame<CrosstermBackend<Stdout>>) -> (Vec<Rect>, Vec<Rect>) {
    let vertical_blocks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(45),
                Constraint::Percentage(25),
                Constraint::Percentage(20),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(frame.size());
    let vertical_middle_block = vertical_blocks[MIDDLE_BLOCK_INDEX];

    let horizontal_blocks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
            ]
            .as_ref(),
        )
        .split(vertical_middle_block);

    return (vertical_blocks, horizontal_blocks);
}

pub struct Menu<'a> {
    menu_state: ListState,
    screen: Screen,
    menu_items_list: List<'a>,
    footer: Paragraph<'a>,
}

impl<'a> Menu<'a> {
    pub fn new() -> Menu<'a> {
        let mut menu_state = ListState::default();
        menu_state.select(Some(0));

        let screen = Screen::new().unwrap();

        let menu_items = lotteries_to_menu_items(&SUPPORTED_LOTTERIES);
        let menu_items_list = List::new(menu_items)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Green),
            )
            .highlight_symbol("          -->  ");

        let footer = get_footer();

        Menu {
            menu_state,
            screen,
            menu_items_list,
            footer,
        }
    }

    fn draw(&mut self) {
        self.screen.show(&|frame| {
            let (vertical_blocks, horyzontal_blocks) = get_layout(frame);
            let horyzontal_middle_block = horyzontal_blocks[MIDDLE_BLOCK_INDEX];
            let last_vertical_block = vertical_blocks.last().unwrap();

            frame.render_stateful_widget(
                self.menu_items_list.clone(),
                horyzontal_middle_block,
                &mut self.menu_state.clone(),
            );

            frame.render_widget(self.footer.clone(), *last_vertical_block);
        });
    }

    pub fn select(&mut self) -> MenuEvent {
        loop {
            self.draw();

            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    if let Some(selected_item) = self.menu_state.selected() {
                        if selected_item == 0 {
                            self.menu_state.select(Some(SUPPORTED_LOTTERIES.len() - 1));
                        } else {
                            self.menu_state.select(Some(selected_item - 1))
                        }
                    }
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    if let Some(selected_item) = self.menu_state.selected() {
                        if selected_item == SUPPORTED_LOTTERIES.len() - 1 {
                            self.menu_state.select(Some(0));
                        } else {
                            self.menu_state.select(Some(selected_item + 1))
                        }
                    }
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    if let Some(selected_item) = self.menu_state.selected() {
                        return MenuEvent::MenuItemSelected(SUPPORTED_LOTTERIES[selected_item]);
                    }
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Char(EXIT_CHAR),
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => return MenuEvent::Shutdown,

                _ => (),
            }
        }
    }
}
