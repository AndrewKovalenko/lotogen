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

    use self::view::oop_show;

    pub fn run() {
        let mut menu = menu::Menu::new();

        loop {
            match menu.select() {
                MenuEvent::MenuItemSelected(lotery) => {
                    let lottery_ticket: LotteryTicket = generate_lottery_ticket(&lotery);

                    oop_show(&lottery_ticket);
                }
                MenuEvent::Shutdown => break,
            }
        }
    }
}
