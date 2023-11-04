use std::fmt;

const DAY_IN_MINUTES: i32 = 24 * 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hour: i32,
    minute: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes_from_midnight = Self::calc_minutes_from_midnight(hours, minutes);

        Self {
            hour: minutes_from_midnight / 60,
            minute: minutes_from_midnight % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes_from_midnight =
            Self::calc_minutes_from_midnight(self.hour, &self.minute + minutes);

        Self {
            hour: minutes_from_midnight / 60,
            minute: minutes_from_midnight % 60,
        }
    }

    pub fn calc_minutes_from_midnight(hours: i32, minutes: i32) -> i32 {
        let total_minutes = (hours * 60) + minutes;
        let mut minutes_from_midnight = total_minutes % DAY_IN_MINUTES;

        if minutes_from_midnight < 0 {
            minutes_from_midnight = DAY_IN_MINUTES + minutes_from_midnight;
        }

        minutes_from_midnight
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}
