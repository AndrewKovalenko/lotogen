extern crate crossterm;

pub mod application {
    mod generator;
    mod lotteries;
    mod menu;
    mod settings;
    mod tickets_view;

    mod ui_components {
        pub mod screen;
    }

    use self::{generator::generate_lottery_ticket, tickets_view::show_results};
    use generator::Ticket;
    use menu::{Menu, MenuEvent};

    const TICKETS_PER_SCREEN: usize = 2;

    pub fn run() {
        loop {
            match Menu::new().select() {
                MenuEvent::MenuItemSelected(lotery) => {
                    let tickets: Vec<Ticket> = (0..TICKETS_PER_SCREEN)
                        .map(|_| generate_lottery_ticket(&lotery))
                        .collect();

                    show_results(&tickets);
                }
                MenuEvent::Shutdown => break,
            }
        }
    }
}
