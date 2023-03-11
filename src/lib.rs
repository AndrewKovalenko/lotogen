#[macro_use]
extern crate crossterm;

const NUMBER_OF_TICKETS: u8 = 2;

pub mod application {
    mod generator;
    mod lotteries;
    mod menu;
    mod view;

    mod ui_components {
        pub mod screen;
    }

    use generator::{generate_lottery_ticket, LotteryTicket};
    use menu::MenuEvent;

    use crate::NUMBER_OF_TICKETS;

    use self::view::show_ticket;

    pub fn run() {
        let mut menu = menu::Menu::new();

        loop {
            match menu.select() {
                MenuEvent::MenuItemSelected(lotery) => {
                    let lottery_ticket: LotteryTicket = generate_lottery_ticket(&lotery);

                    show_ticket(&lottery_ticket, NUMBER_OF_TICKETS);
                }
                MenuEvent::Shutdown => break,
            }
        }
    }
}
