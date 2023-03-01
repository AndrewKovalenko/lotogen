#[derive(Clone, Copy)]
pub enum Lottery<'a> {
    PowerBall(&'a str),
    MegaMillions(&'a str),
}
