use std::fmt::{self, Formatter};

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

fn roll_over_time(hours: i32, minutes: i32) -> (i32, i32) {
    let total_minutes = (hours * 60) + minutes;
    let total_minutes = ((total_minutes % 1440) + 1440) % 1440;

    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;

    (hours, minutes)
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours, minutes) = roll_over_time(hours, minutes);
        Clock { hours, minutes }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let hours: String = match self.hours {
            24 => "00".to_string(),
            _ => {
                if self.hours > 9 {
                    self.hours.to_string()
                } else {
                    "0".to_string() + &self.hours.to_string()
                }
            }
        };

        let minutes = if self.minutes > 9 {
            self.minutes.to_string()
        } else {
            "0".to_string() + &self.minutes.to_string()
        };

        write!(f, "{}:{}", hours, minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}