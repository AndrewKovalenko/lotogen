#[macro_use]
extern crate crossterm;

pub mod application {
    mod generator;
    mod lotteries;
    mod menu;
    mod view;

    mod ui_components {
        pub mod block;
        pub mod screen;
    }

    use generator::{generate_lottery_ticket, LotteryTicket};
    use menu::MenuEvent;

    use self::view::show_ticket;

    pub fn run() {
        let mut menu = menu::Menu::new();

        loop {
            match menu.select() {
                MenuEvent::MenuItemSelected(lotery) => {
                    let lottery_ticket: LotteryTicket = generate_lottery_ticket(&lotery);

                    show_ticket(&lottery_ticket, 2);
                }
                MenuEvent::Shutdown => break,
            }
        }
    }
}
