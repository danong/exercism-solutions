use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        const MINS_PER_DAY: i32 = 24 * 60;
        
        // mins_total must be in range [0, MINS_PER_day).
        let mins_total = (hours * 60 + minutes).rem_euclid(MINS_PER_DAY);
             
        Clock {
            hours: mins_total / 60,
            minutes: mins_total % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // Delegate (under|over)flow logic to constructor.
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
