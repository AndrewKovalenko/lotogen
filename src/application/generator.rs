use rand::Rng;

use super::lotteries::{Lottery, Namable};

struct GeneratorSettingsForLottery {
    main_field_min_number: u8,
    main_field_max_number: u8,

    wining_numbers_count: u8,
    separate_number_min: u8,
    separate_number_max: u8,
}

pub struct LotteryTicket {
    pub lottery_name: String,
    pub main_field: Vec<i8>,
    pub separate_number: Vec<i8>,
}

fn get_generator_settings(lottery: &Lottery) -> GeneratorSettingsForLottery {
    match lottery {
        Lottery::PowerBall(_) => GeneratorSettingsForLottery {
            main_field_min_number: 1,
            main_field_max_number: 69,
            wining_numbers_count: 5,

            separate_number_min: 1,
            separate_number_max: 26,
        },
        Lottery::MegaMillions(_) => GeneratorSettingsForLottery {
            main_field_min_number: 1,
            main_field_max_number: 70,
            wining_numbers_count: 5,

            separate_number_min: 1,
            separate_number_max: 25,
        },
    }
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

fn get_lottery_name<'a, T>(lottery: &'a T) -> &'a str
where
    T: Namable,
{
    return lottery.get_name();
}

pub fn generate_lottery_ticket(lottery: &Lottery) -> LotteryTicket {
    let generator_settings = get_generator_settings(lottery);

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

    let lottery_name = get_lottery_name(lottery).to_owned();

    return LotteryTicket {
        lottery_name: lottery_name,
        main_field: main_field_numbers,
        separate_number: separate_number_field,
    };
}
