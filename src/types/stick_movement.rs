use std::fmt;
use std::fmt::Formatter;

pub struct StickMovement(pub i16, pub i16);

impl fmt::Display for StickMovement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.0, self.1)
    }
}
