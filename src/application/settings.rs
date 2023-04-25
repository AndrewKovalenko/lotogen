use super::lotteries::Lottery;

pub struct LotterySettings {
    pub main_field_min_number: u8,
    pub main_field_max_number: u8,
    pub main_field_offset: usize,

    pub wining_numbers_count: u8,
    pub separate_number_min: u8,
    pub separate_number_max: u8,
    pub separate_number_offset: usize,
}

pub fn get_lottery_settings(lottery: &Lottery) -> LotterySettings {
    match lottery {
        Lottery::PowerBall(_) => LotterySettings {
            main_field_min_number: 1,
            main_field_max_number: 69,
            wining_numbers_count: 5,
            main_field_offset: 3,

            separate_number_min: 1,
            separate_number_max: 26,
            separate_number_offset: 3,
        },
        Lottery::MegaMillions(_) => LotterySettings {
            main_field_min_number: 1,
            main_field_max_number: 70,
            wining_numbers_count: 5,
            main_field_offset: 6,

            separate_number_min: 1,
            separate_number_max: 25,
            separate_number_offset: 3,
        },
    }
}
