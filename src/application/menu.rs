use super::lotteries::Lottery;
use super::ui_components::screen::Screen;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use tui::style::{Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem, ListState};

pub enum MenuEvent<'a> {
    MenuItemSelected(Lottery<'a>),
    Shutdown,
}

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

pub struct Menu<'a> {
    menu_state: ListState,
    screen: Screen,
    menu_items_list: List<'a>,
}

impl<'a> Menu<'a> {
    pub fn new() -> Menu<'a> {
        let mut menu_state = ListState::default();
        menu_state.select(Some(0));

        let screen = Screen::new().unwrap();

        let menu_items = lotteries_to_menu_items(&SUPPORTED_LOTTERIES);
        let menu_items_list = List::new(menu_items)
            .block(
                Block::default()
                    .title("Select lottery")
                    .borders(Borders::ALL),
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("-->");

        Menu {
            menu_state,
            screen,
            menu_items_list,
        }
    }

    fn draw(&mut self) {
        self.screen.show(&|frame| {
            frame.render_stateful_widget(
                self.menu_items_list.clone(),
                frame.size(),
                &mut self.menu_state.clone(),
            )
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
