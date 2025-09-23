use crossterm::event::read;
use crossterm::event::{Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }
    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("Goodbye.\r\n");
    }
    fn repl(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?; //The operator ? enable us to return the error immediately if it occurs
        loop {
            if let Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) = read()?
            {
                println!(
                    "Key Event: code={:?}, modifiers={:?}, kind={:?}, state={:?}\r",
                    code, modifiers, kind, state
                );
                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    }
                    _ => (),
                }
            }
            if self.should_quit {
                break;
            }
        }

        disable_raw_mode()?;
        Ok(())
    }
}
