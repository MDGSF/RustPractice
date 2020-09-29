use std::fmt;

#[derive(Eq, PartialEq, Debug)]
pub struct Clock {
  hours: i32,
  minutes: i32,
}

impl fmt::Display for Clock {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:02}:{:02}", self.hours, self.minutes)
  }
}

impl Clock {
  pub fn new(mut hours: i32, mut minutes: i32) -> Self {
    let mut borrow_hours = 0;
    if minutes < 0 {
      borrow_hours = -minutes / 60;
      if (-minutes % 60) > 0 {
        borrow_hours += 1;
      }
      minutes = 60 - (-minutes % 60);
      minutes %= 60;
    }
    hours -= borrow_hours;
    if hours < 0 {
      hours = 24 - (-hours % 24);
    }
    hours += minutes / 60;
    minutes %= 60;
    hours %= 24;
    Clock { hours, minutes }
  }

  pub fn add_minutes(&self, minutes: i32) -> Self {
    Clock::new(self.hours, self.minutes + minutes)
  }
}
