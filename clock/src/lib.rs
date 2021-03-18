use std::fmt;
use std::ops::{Div, Rem};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let h_from_min = minutes.div(60);
        let h_total = h_from_min + hours;
        let mut hours_new = h_total.rem(24);
        let mut minutes_new = minutes.rem(60);
        if minutes_new < 0 {
            minutes_new += 60;
            hours_new -= 1;
        }
        if hours_new < 0 {
            hours_new += 24;
        }
        Clock {
            hours: hours_new,
            minutes: minutes_new,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let h = minutes.div(60);
        let mins = minutes.rem(60);
        Clock::new(self.hours + h, self.minutes + mins)
    }
}
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
