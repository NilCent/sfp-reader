use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, terminal};
use std::io;
use std::io::Read;
use std::time::Duration;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("could not disable raw mode");
    }
}
fn main() -> io::Result<()> {
    let _cleanup = CleanUp;
    terminal::enable_raw_mode()?;
    loop {
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(event) = event::read()? {
                match event {
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => break,
                    _ => {}
                }
                println!("{:?}\r", event);
            }
        } else {
            println!("no input yet\r");
        }
    }
    Ok(())
}
