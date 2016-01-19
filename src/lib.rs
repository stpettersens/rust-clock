/* 
    A hh:mm:ss clock for the CLI.
    Author: Sam Saint-Pettersen <s.stpettesen+github@gmail.com> 
*/

pub struct Clock {
    hours: i64,
    minutes: i64,
    seconds: i64,
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
    pub fn set_time_ms(&mut self, ms: i64) {
        self.seconds = (ms / 1000) % 60;
        self.minutes = (ms / (1000 * 60)) % 60;
        self.hours = (ms / (1000 * 60 * 60)) % 24;
    }
    pub fn set_time_s(&mut self, seconds: i64) {
        self.set_time_ms(seconds / 1000);
    }
    pub fn get_time(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds)
    }
}
