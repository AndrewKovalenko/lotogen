use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{Print, SetForegroundColor};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{stdout, Stdout};

const LOTERIES: [&str; 2] = ["Power Ball", "Mega Millions"];
const EXIT_CHAR: char = 'q';

pub struct Menu {
    selected_item_index: usize,
    stdout_instance: Stdout,
}

impl Menu {
    pub fn new() -> Menu {
        enable_raw_mode().unwrap();

        Menu {
            selected_item_index: 0,
            stdout_instance: stdout(),
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

        LOTERIES.iter().enumerate().for_each(|(index, menu_item)| {
            let color = if self.selected_item_index == index {
                crossterm::style::Color::Green
            } else {
                crossterm::style::Color::Grey
            };

            Print(menu_item);
            execute!(
                self.stdout_instance,
                cursor::MoveTo(40, 20 + index as u16),
                SetForegroundColor(color),
                Print(menu_item),
            )
            .unwrap();
        });
    }

    pub fn select(&mut self) {
        loop {
            self.draw();

            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    if self.selected_item_index == 0 {
                        self.selected_item_index = LOTERIES.len() - 1;
                    } else {
                        self.selected_item_index = self.selected_item_index - 1
                    }
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    if self.selected_item_index == LOTERIES.len() - 1 {
                        self.selected_item_index = 0;
                    } else {
                        self.selected_item_index = self.selected_item_index + 1;
                    }
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Char(EXIT_CHAR),
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => break,

                _ => (),
            }
        }
    }
}

impl Drop for Menu {
    fn drop(&mut self) {
        execute!(self.stdout_instance, cursor::Show).unwrap();

        disable_raw_mode().unwrap();
    }
}
