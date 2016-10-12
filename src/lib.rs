/* 
    rust-clock
    Copyright (c) 2016 Sam Saint-Pettersen.

    Released under the MIT License.
*/

//! Rust library to create a hh:mm:ss clock for the CLI from seconds or miliseconds.

pub struct Clock {
    hours: i64,
    minutes: i64,
    seconds: i64,
}

impl Clock {

    ///
    /// Create a new clock.
    ///
    pub fn new() -> Clock {
        Clock {
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }

    /// 
    /// Set the clock time in milliseconds.
    /// 
    /// * `ms` - Milliseconds to set time from.
    pub fn set_time_ms(&mut self, ms: i64) {
        self.seconds = (ms / 1000) % 60;
        self.minutes = (ms / (1000 * 60)) % 60;
        self.hours = (ms / (1000 * 60 * 60)) % 24;
    }

    ///
    /// Set the clock time in seconds.
    ///
    /// * `seconds` - Seconds to set time from.
    pub fn set_time_secs(&mut self, seconds: i64) {
        self.set_time_ms(seconds * 1000);
    }

    ///
    /// Get the clock time in hh:mm:ss notation.
    ///
    pub fn get_time(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds)
    }
}

#[cfg(test)]
#[test]
fn test_clock_ms() {
    let mut clock = Clock::new();
    clock.set_time_ms(60000);
    assert_eq!(clock.get_time(), "00:01:00");
}

#[test]
fn test_clock_secs() {
    let mut clock = Clock::new();
    clock.set_time_secs(330);
    assert_eq!(clock.get_time(), "00:05:30");
}
