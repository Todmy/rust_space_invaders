use std::{time::Duration, cmp::max};
use rusty_time::Timer;
use crate::{NUM_COLS, NUM_ROWS, frame::{Drawable, Frame}};

pub struct Invader {
  pub x: usize,
  pub y: usize,
}

pub struct Invaders {
  pub army: Vec<Invader>,
  pub direction: i32,
  pub move_timer: Timer,
}

impl Invaders {
  pub fn new() -> Self {
    let mut army = Vec::new();
    for x in 0..NUM_COLS {
      for y in 0..NUM_ROWS {
        if x > 1 && x < NUM_COLS - 2 && y > 0 && y < 9 && x % 2 == 0 && y % 2 == 0 {
          army.push(Invader { x, y });
        }
      }
    }
    Self {
      army,
      direction: 1,
      move_timer: Timer::new(Duration::from_millis(5000)),
    }
  }
  
  pub fn update(&mut self, delta: Duration) -> bool {
    self.move_timer.tick(delta);
    if self.move_timer.just_finished() {
      self.move_timer.reset();
      let mut downwards = false;
      if self.direction == -1 {
        let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
        if min_x == 0 {
          self.direction = 1;
          downwards = true;
        }
      } else {
        let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
        if max_x == NUM_COLS - 1 {
          self.direction = -1;
          downwards = true;
        }
      }
      
      if downwards {
        let new_duration = max(self.move_timer.duration().as_millis() - 250, 250);
        self.move_timer = Timer::new(Duration::from_millis(new_duration as u64));
        for invader in self.army.iter_mut() {
          invader.y += 1;
        }
      } else {
        for invader in self.army.iter_mut() {
          invader.x = ((invader.x as i32) + self.direction) as usize;
        }
      }
      return true;
    }
    false
  }
}

impl Drawable for Invaders {
  fn draw(&self, frame: &mut Frame) {
    for invader in self.army.iter() {
      frame[invader.x][invader.y] =
      if (self.move_timer.percent_left() * 100.0) as u32 % 20 < 10
      { "x" } else { "+" };
    }
  }
}