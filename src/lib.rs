extern crate crossterm;

pub mod application {
    mod generator;
    mod lotteries;
    mod menu;
    mod tickets_view;

    mod ui_components {
        pub mod screen;
    }

    use self::{generator::generate_lottery_ticket, tickets_view::show_ticket};
    use generator::Ticket;
    use menu::{Menu, MenuEvent};

    pub fn run() {
        loop {
            match Menu::new().select() {
                MenuEvent::MenuItemSelected(lotery) => {
                    let lottery_ticket: Ticket = generate_lottery_ticket(&lotery);

                    show_ticket(&lottery_ticket);
                }
                MenuEvent::Shutdown => break,
            }
        }
    }
}
