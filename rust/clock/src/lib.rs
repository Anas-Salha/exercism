use std::fmt;

const DAY_IN_MINUTES: i32 = 24 * 60;
const HOUR_IN_MINUTES: i32 = 60;
#[derive(Debug, PartialEq)]
pub struct Clock {
    minute: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minute: ((hours * HOUR_IN_MINUTES) + minutes).rem_euclid(DAY_IN_MINUTES),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            minute: (&self.minute + minutes).rem_euclid(DAY_IN_MINUTES),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minute / HOUR_IN_MINUTES,
            self.minute % HOUR_IN_MINUTES
        )
    }
}
