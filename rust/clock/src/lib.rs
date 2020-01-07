use std::fmt;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

// I could also implement this with deriving.
impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Reuse the fmt() function from the fmt::Display trait since they return the same thing.
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

// Normalize the hours and minutes on a clock so that the hours is between 0 and 23, and the
// minutes is between 0 and 59.
fn normalize(clock: Clock) -> Clock {
    let quot_mins = clock.minutes.div_euclid(60);
    let rem_mins = clock.minutes.rem_euclid(60);
    let new_hours = (clock.hours + quot_mins).rem_euclid(24);
    Clock { hours: new_hours, minutes: rem_mins }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        normalize(Clock { hours, minutes })
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        normalize(Clock { hours: self.hours, minutes: self.minutes + minutes })
    }
}
