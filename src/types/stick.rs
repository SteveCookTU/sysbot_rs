use std::fmt;
use std::fmt::Formatter;

pub enum Stick {
    RIGHT,
    LEFT,
}

impl fmt::Display for Stick {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Stick::RIGHT => write!(f, "RSTICK"),
            Stick::LEFT => write!(f, "LSTICK"),
        }
    }
}
