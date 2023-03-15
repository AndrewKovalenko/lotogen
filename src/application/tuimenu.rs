use std::io::Stdout;

use super::lotteries::Lottery;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use tui::{
    backend::CrosstermBackend,
    style::Style,
    widgets::{List, ListItem, ListState},
    Frame,
};

pub enum MenuItem<'a> {
    Lottery(Lottery<'a>),
    Action(&'a str),
}

const EXIT_CHAR: char = 'q';

pub const QUIT: &str = "Quit";
pub const MENU_ITEMS: &'static [MenuItem] = &[
    MenuItem::Lottery(Lottery::MegaMillions("Mega Million")),
    MenuItem::Lottery(Lottery::PowerBall("PowerBall")),
    MenuItem::Action(QUIT),
];

pub struct Menu<'a, 'b> {
    frame: &'a mut Frame<'b, CrosstermBackend<Stdout>>,
    menu_state: ListState,
}

impl<'a, 'b> Menu<'a, 'b> {
    pub fn new(frame: &'a mut Frame<'b, CrosstermBackend<Stdout>>) -> Menu<'a, 'b> {
        let mut menu_state = ListState::default();
        menu_state.select(Some(0));

        return Menu { frame, menu_state };
    }

    fn show_menu(&mut self) {
        let menu_items: Vec<ListItem> = MENU_ITEMS
            .iter()
            .map(|m| {
                let item_text = match *m {
                    MenuItem::Lottery(Lottery::MegaMillions(name)) => name,
                    MenuItem::Lottery(Lottery::PowerBall(name)) => name,
                    MenuItem::Action(name) => name,
                };

                return ListItem::new(item_text);
            })
            .collect();

        let menu = List::new(menu_items)
            .style(Style::default())
            .highlight_symbol(">>");
        self.frame
            .render_stateful_widget(menu, self.frame.size(), &mut self.menu_state);
    }

    pub fn select(&mut self) -> &MenuItem {
        loop {
            self.show_menu();

            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    if self.menu_state.selected().unwrap() == 0 {
                        self.menu_state.select(Some(MENU_ITEMS.len() - 1));
                    } else {
                        let previuosly_selected = self.menu_state.selected().unwrap();
                        self.menu_state.select(Some(previuosly_selected - 1));
                    }
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    if self.menu_state.selected().unwrap() == MENU_ITEMS.len() - 1 {
                        self.menu_state.select(Some(0));
                    } else {
                        let previuosly_selected = self.menu_state.selected().unwrap();
                        self.menu_state.select(Some(previuosly_selected + 1));
                    }
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    let selected_item = self.menu_state.selected().unwrap();
                    return &MENU_ITEMS[selected_item];
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Char(EXIT_CHAR),
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => return &MenuItem::Action(QUIT),

                _ => (),
            }
        }
    }
}
