#[macro_use]
extern crate crossterm;

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

    use self::view::show_ticket;
    const NUMBER_OF_TICKETS: usize = 2;

    pub fn run() {
        let mut menu = menu::Menu::new();

        loop {
            match menu.select() {
                MenuEvent::MenuItemSelected(lotery) => {
                    let mut tickets: Vec<LotteryTicket> = vec![];

                    for _ in 1..=NUMBER_OF_TICKETS {
                        let lottery_ticket: LotteryTicket = generate_lottery_ticket(&lotery);
                        tickets.push(lottery_ticket);
                    }

                    show_ticket(&tickets, NUMBER_OF_TICKETS);
                }
                MenuEvent::Shutdown => break,
            }
        }
    }
}
