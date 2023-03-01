use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{Print, SetForegroundColor};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{stdout, Stdout};

#[derive(Clone, Copy)]
pub enum Lottery<'a> {
    PowerBall(&'a str),
    MegaMillions(&'a str),
}

pub enum MenuEvent<'a> {
    MenuItemSelected(Lottery<'a>),
    Shutdown,
}

const EXIT_CHAR: char = 'q';

pub struct Menu<'a> {
    selected_item_index: usize,
    stdout_instance: Stdout,
    lotteries: Vec<Lottery<'a>>,
}

impl<'a> Menu<'a> {
    pub fn new() -> Menu<'a> {
        enable_raw_mode().unwrap();

        Menu {
            selected_item_index: 0,
            stdout_instance: stdout(),
            lotteries: vec![
                Lottery::PowerBall("Power Ball"),
                Lottery::MegaMillions("Mega Millions"),
            ],
        }
    }

    fn draw(&mut self) {
        execute!(
            self.stdout_instance,
            Clear(ClearType::All),
            cursor::MoveTo(40, 20),
            cursor::Hide,
        )
        .unwrap();

        self.lotteries
            .iter()
            .enumerate()
            .for_each(|(index, menu_item)| {
                let color = if self.selected_item_index == index {
                    crossterm::style::Color::Green
                } else {
                    crossterm::style::Color::Grey
                };

                let lottery_name = match *menu_item {
                    Lottery::MegaMillions(name) => name,
                    Lottery::PowerBall(name) => name,
                };

                execute!(
                    self.stdout_instance,
                    cursor::MoveTo(40, 20 + index as u16),
                    SetForegroundColor(color),
                    Print(lottery_name),
                )
                .unwrap();
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
                    if self.selected_item_index == 0 {
                        self.selected_item_index = self.lotteries.len() - 1;
                    } else {
                        self.selected_item_index = self.selected_item_index - 1
                    }
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    if self.selected_item_index == self.lotteries.len() - 1 {
                        self.selected_item_index = 0;
                    } else {
                        self.selected_item_index = self.selected_item_index + 1;
                    }
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => return MenuEvent::MenuItemSelected(self.lotteries[self.selected_item_index]),

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

impl<'a> Drop for Menu<'a> {
    fn drop(&mut self) {
        execute!(self.stdout_instance, cursor::Show).unwrap();

        disable_raw_mode().unwrap();
    }
}
