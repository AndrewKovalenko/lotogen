#[derive(Clone, Copy)]
pub enum Lottery<'a> {
    PowerBall(&'a str),
    MegaMillions(&'a str),
}

pub trait Namable {
    fn get_name(&self) -> &str;
}

impl<'a> Namable for Lottery<'a> {
    fn get_name(&self) -> &str {
        return match *self {
            Self::PowerBall(name) => name,
            Self::MegaMillions(name) => name,
        };
    }
}
