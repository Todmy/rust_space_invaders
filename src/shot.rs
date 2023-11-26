use std::time::Duration;

use rusty_time::Timer;

use crate::frame::{Frame, Drawable};

pub struct Shot {
  pub x: usize,
  pub y: usize,
  pub exploded: bool,
  timer: Timer,
}

impl Shot {
  pub fn new(x: usize, y: usize) -> Self {
    Self {
      x,
      y,
      exploded: false,
      timer: Timer::new(Duration::from_millis(50)),
    }
  }
  
  pub fn update(&mut self, delta: Duration) {
    self.timer.tick(delta);
    if self.timer.finished() && !self.exploded {
      if self.y > 0 {
        self.y -= 1;
      } else {
        self.exploded = true;
      }
      self.timer.reset();
    }
  }
  
  pub fn explode(&mut self) {
    self.exploded = true;
    self.timer = Timer::new(Duration::from_millis(250));
  }
  
  pub fn dead(&self) -> bool {
    (self.exploded && self.timer.finished()) || (self.y == 0)
  }
}

impl Drawable for Shot {
  fn draw(&self, frame: &mut Frame) {
    frame[self.x][self.y] = if self.exploded { " " } else { "|" };
  }
}