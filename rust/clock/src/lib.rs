use core::fmt;

const MIN_IN_HOUR: i32 = 60;
const HOURS_IN_DAY: i32 = 24;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours, minutes }.resolve_negative_and_overflowing_values()
    }

    fn resolve_negative_and_overflowing_values(mut self) -> Self {
        if self.minutes < 0 {
            let hours_overflow = self.minutes.abs().div_euclid(MIN_IN_HOUR) + 1;
            self.minutes = MIN_IN_HOUR - self.minutes.abs().rem_euclid(MIN_IN_HOUR);
            self.hours -= hours_overflow;
        }

        if self.hours < 0 {
            self.hours = HOURS_IN_DAY - self.hours.abs().rem_euclid(HOURS_IN_DAY);
        }

        if self.minutes >= 60 {
            let hours_overflow = self.minutes.div_euclid(MIN_IN_HOUR);
            self.minutes = self.minutes.rem_euclid(MIN_IN_HOUR);
            self.hours += hours_overflow
        }

        self
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        self.minutes += minutes;
        return self.resolve_negative_and_overflowing_values();
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours.rem_euclid(HOURS_IN_DAY), self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
