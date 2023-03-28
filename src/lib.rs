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

    use self::{generator::generate_lottery_ticket, view::show_ticket};
    use generator::LotteryTicket;
    use menu::{Menu, MenuEvent};

    pub fn run() {
        let mut menu = Menu::new();
        loop {
            match menu.select() {
                MenuEvent::MenuItemSelected(lotery) => {
                    let lottery_ticket: LotteryTicket = generate_lottery_ticket(&lotery);

                    show_ticket(&lottery_ticket, 2);
                }
                MenuEvent::Shutdown => break,
                _ => (),
            }
        }
    }
}
