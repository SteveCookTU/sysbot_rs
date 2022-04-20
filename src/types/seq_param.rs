use crate::types::button::Button;
use crate::types::stick_movement::StickMovement;
use std::fmt;
use std::fmt::Formatter;

pub enum SeqParam {
    Click(Button),
    Press(Button),
    Release(Button),
    MoveRight(StickMovement),
    MoveLeft(StickMovement),
    Wait(u32),
}

impl fmt::Display for SeqParam {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SeqParam::Click(b) => {
                write!(f, "{}", b)
            }
            SeqParam::Press(b) => {
                write!(f, "+{}", b)
            }
            SeqParam::Release(b) => {
                write!(f, "-{}", b)
            }
            SeqParam::MoveLeft(mv) => {
                write!(f, "%{}", mv)
            }
            SeqParam::MoveRight(mv) => {
                write!(f, "&{}", mv)
            }
            SeqParam::Wait(t) => {
                write!(f, "{}", format_args!("{}{}", "W", t))
            }
        }
    }
}
