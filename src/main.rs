use std::{error::Error, time::{Duration, Instant}, sync::mpsc, thread};
use rusty_audio::Audio;
use space_invadors::{
  frame::{self, Drawable}, 
  render, 
  player::Player, 
  invaders::Invaders
};
use std::io;
use crossterm::{
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
  cursor::{Hide, Show}, 
  ExecutableCommand, 
  event::{self, Event, KeyCode}
};

fn main() -> Result<(), Box<dyn Error>> {
  let mut audio = Audio::new();
  audio.add("explode", "assets/explode.wav");
  audio.add("lose", "assets/lose.wav");
  audio.add("move", "assets/move.wav");
  audio.add("pew", "assets/pew.wav");
  audio.add("startup", "assets/startup.wav");
  audio.add("win", "assets/win.wav");
  audio.play("startup");
  
  // Terminal input
  let mut stdout = io::stdout();
  terminal::enable_raw_mode()?;
  stdout.execute(EnterAlternateScreen)?;
  stdout.execute(Hide)?;
  
  // render loop in a separate thread
  let (render_tx, render_rx) = mpsc::channel();
  let render_handle = thread::spawn(move || {
    let mut last_frame = frame::new_frame();
    let mut stdout = io::stdout();
    render::render(&mut stdout, &last_frame, &last_frame, true);
    loop {
      let curr_frame = match render_rx.recv() {
        Ok(x) => x,
        Err(_) => break,
      };
      render::render(&mut stdout, &last_frame, &curr_frame, false);
      last_frame = curr_frame;
    }
  });
  
  let mut player = Player::new();
  let mut instant = Instant::now();
  let mut invaders = Invaders::new();
  // Game loop
  'gameloop: loop {
    let delta = instant.elapsed();
    instant = Instant::now();
    // Per-frame init
    let mut curr_frame = frame::new_frame();
    // Handle input
    while event::poll(Duration::default())? {
      if let Event::Key(key_event) = event::read()? {
        match key_event.code {
          KeyCode::Esc | KeyCode::Char('q') => {
            audio.play("lose");
            break 'gameloop;
          }
          KeyCode::Left => {
            player.move_left();
          }
          KeyCode::Right => {
            player.move_right();
          }
          KeyCode::Char(' ') | KeyCode::Enter => {
            if player.shoot() {
              audio.play("pew");
            }
          }
          _ => {}
        }
      }
    }
    
    // Update
    player.update(delta);
    if invaders.update(delta) {
      audio.play("move");
    }
    if player.detect_hits(&mut invaders) {
      audio.play("explode");
    }
    
    // Render
    let drawbles: Vec<&dyn Drawable> = vec![&player, &invaders];
    drawbles.iter().for_each(|d| d.draw(&mut curr_frame));
    let _ = render_tx.send(curr_frame);
    thread::sleep(Duration::from_millis(1));

    // Check for win/lose
    if invaders.all_killed() {
      audio.play("win");
      break 'gameloop;
    } else if invaders.reached_bottom() {
      audio.play("lose");
      break 'gameloop;
    }
  }
  
  // cleanup
  stdout.execute(Show)?;
  stdout.execute(LeaveAlternateScreen)?;
  terminal::disable_raw_mode()?;
  
  drop(render_tx);
  render_handle.join().unwrap();
  audio.wait();
  
  Ok(())
}
