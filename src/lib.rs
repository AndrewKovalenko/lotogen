extern crate crossterm;

pub mod application {
    mod generator;
    mod lotteries;
    mod menu;
    mod view;

    mod ui_components {
        pub mod screen;
    }

    use self::{generator::generate_lottery_ticket, view::show_ticket};
    use generator::LotteryTicket;
    use menu::{Menu, MenuEvent};

    pub fn run() {
        loop {
            match Menu::new().select() {
                MenuEvent::MenuItemSelected(lotery) => {
                    let lottery_ticket: LotteryTicket = generate_lottery_ticket(&lotery);

                    show_ticket(&lottery_ticket);
                }
                MenuEvent::Shutdown => break,
            }
        }
    }
}
