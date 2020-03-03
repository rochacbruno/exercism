use std::fmt;

const HOURS_IN_A_DAY: i32 = 24;
const MINUTES_IN_HOUR: i32 = 60;
const MINUTES_IN_A_DAY: i32 = MINUTES_IN_HOUR * HOURS_IN_A_DAY;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    time_in_minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            time_in_minutes: Self::calculate(hours * MINUTES_IN_HOUR + minutes),
        }
    }

    pub fn hours(&self) -> i32 {
        self.time_in_minutes / MINUTES_IN_HOUR
    }

    pub fn minutes(&self) -> i32 {
        self.time_in_minutes % MINUTES_IN_HOUR
    }

    fn calculate(total_minutes: i32) -> i32 {
        ((total_minutes % MINUTES_IN_A_DAY) + MINUTES_IN_A_DAY) % MINUTES_IN_A_DAY
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            time_in_minutes: Self::calculate(self.time_in_minutes + minutes),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours(), self.minutes())
    }
}
