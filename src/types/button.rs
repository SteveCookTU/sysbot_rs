use crate::types::stick::Stick;
use std::fmt;
use std::fmt::Formatter;

pub enum Button {
    A,
    B,
    X,
    Y,
    STICK(Stick),
    L,
    R,
    ZL,
    ZR,
    PLUS,
    MINUS,
    DLEFT,
    DUP,
    DDOWN,
    DRIGHT,
    HOME,
    CAPTURE,
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Button::A => write!(f, "A"),
            Button::B => write!(f, "B"),
            Button::X => write!(f, "X"),
            Button::Y => write!(f, "Y"),
            Button::STICK(stick) => write!(f, "{}", stick),
            Button::L => write!(f, "L"),
            Button::R => write!(f, "R"),
            Button::ZL => write!(f, "ZL"),
            Button::ZR => write!(f, "ZR"),
            Button::PLUS => write!(f, "PLUS"),
            Button::MINUS => write!(f, "MINUS"),
            Button::DLEFT => write!(f, "DL"),
            Button::DUP => write!(f, "DU"),
            Button::DRIGHT => write!(f, "DR"),
            Button::DDOWN => write!(f, "DD"),
            Button::HOME => write!(f, "HOME"),
            Button::CAPTURE => write!(f, "CAPTURE"),
        }
    }
}
