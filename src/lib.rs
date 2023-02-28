#[macro_use]
extern crate crossterm;

pub mod application {
    mod menu;

    pub fn run() {
        let mut menu = menu::Menu::new();

        loop {
            menu.select();
        }
    }
}
