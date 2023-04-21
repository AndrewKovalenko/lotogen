use rand::Rng;

use super::lotteries::Lottery;
use super::settings::get_lottery_settings;

pub const GAMES_PER_TICKET: usize = 5;

pub struct Game {
    pub main_field: Vec<i8>,
    pub separate_number: Vec<i8>,
}

pub struct Ticket<'a> {
    pub lottery: Lottery<'a>,
    pub games: Vec<Game>,
}

fn generate_ticket_field(range_min: u8, rannge_max: u8, count: u8) -> Vec<i8> {
    let mut main_field_numbers: Vec<i8> = (range_min..=rannge_max)
        .into_iter()
        .map(|n| n as i8)
        .collect();

    let mut numbers_left = count;
    loop {
        let wining_number = rand::thread_rng().gen_range(range_min..rannge_max) as usize;

        if main_field_numbers[wining_number] > 0 {
            main_field_numbers[wining_number] = (main_field_numbers[wining_number] * -1) as i8;
            numbers_left = numbers_left - 1;
        }

        if numbers_left == 0 {
            break;
        }
    }

    return main_field_numbers;
}

pub fn generate_lottery_ticket<'a>(lottery: &'a Lottery) -> Ticket<'a> {
    let games: Vec<Game> = (0..GAMES_PER_TICKET)
        .map(|_| {
            let generator_settings = get_lottery_settings(lottery);

            let main_field_numbers = generate_ticket_field(
                generator_settings.main_field_min_number,
                generator_settings.main_field_max_number,
                generator_settings.wining_numbers_count,
            );

            let separate_number_field = generate_ticket_field(
                generator_settings.separate_number_min,
                generator_settings.separate_number_max,
                1,
            );

            return Game {
                main_field: main_field_numbers,
                separate_number: separate_number_field,
            };
        })
        .collect();

    Ticket {
        lottery: *lottery,
        games,
    }
}
