#[macro_use]
extern crate crossterm;

pub mod application {
    mod generator;
    mod lotteries;
    mod menu;
    mod tuimenu;
    mod view;

    mod ui_components {
        pub mod screen;
    }

    use generator::{generate_lottery_ticket, LotteryTicket};

    use self::{
        tuimenu::{MenuItem, QUIT},
        ui_components::screen,
        view::show_ticket,
    };

    pub fn run() {
        screen::Screen::new().unwrap().show(&|frame| {
            let mut menu = tuimenu::Menu::new(frame);

            loop {
                match menu.select() {
                    MenuItem::Lottery(lotery) => {
                        let lottery_ticket: LotteryTicket = generate_lottery_ticket(&lotery);

                        show_ticket(&lottery_ticket, 2);
                    }
                    MenuItem::Action(QUIT) => break,
                    _ => (),
                }
            }
        });
    }
}
