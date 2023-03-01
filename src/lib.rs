#[macro_use]
extern crate crossterm;

pub mod application {
    mod generator;
    mod lotteries;
    mod menu;

    use generator::generate_lottery_ticket;
    use menu::MenuEvent;

    pub fn run() {
        let mut menu = menu::Menu::new();

        loop {
            match menu.select() {
                MenuEvent::MenuItemSelected(lotery) => {
                    let generator = generate_lottery_ticket(&lotery);
                }
                MenuEvent::Shutdown => break,
            }
        }
    }
}
