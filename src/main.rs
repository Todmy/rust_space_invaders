use std::{error::Error, time::Duration};
use rusty_audio::Audio;
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

    // Game loop
    'gameloop: loop {
        // Handle input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
    }

    // cleanup
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    audio.wait();

    Ok(())
}
