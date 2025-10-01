use crossterm::event::{Event, KeyEvent, KeyEventKind, read};
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};
mod terminal;
mod view;
use terminal::Terminal;
use view::View;

mod commands;
use commands::EditorCommand;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;

        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }
        Ok(Self {
            should_quit: false,
            view,
        })
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }
        }
    }

    // needless_pass_by_value: Event is not huge, so there is not a
    // performance overhead in passing by value, and pattern matching in this
    // function would be needlessly complicated if we pass by reference here.

#[allow(clippy::needless_pass_by_value)]
fn evaluate_event(&mut self, event: Event) {
    let should_process = match &event {
        Event::Key(KeyEvent { kind, .. }) => {
            // Process both Press and Release events, but handle them differently
            matches!(kind, KeyEventKind::Press | KeyEventKind::Release)
        }
        Event::Resize(_, _) => true,
        Event::FocusGained | Event::FocusLost => true, // Add this to handle focus events
        _ => false,
    };

    if should_process {
        match &event {
            Event::Key(KeyEvent {
                kind: KeyEventKind::Release,
                ..
            }) => {
                // Handle key release events - currently just acknowledge them
                // For now, we don't perform any action on release
                // but this prevents panicking in systems that register release events like Windows
            }
            Event::FocusGained | Event::FocusLost => {
                // Handle focus events silently - no action needed for now
            }
            _ => {
                // Handle press events and resize events
                match EditorCommand::try_from(event) {
                    Ok(command) => {
                        if matches!(command, EditorCommand::Quit) {
                            self.should_quit = true;
                        } else {
                            self.view.handle_command(command);
                        }
                    }
                    Err(_err) => {
                        // Silently ignore unrecognized commands to prevent terminal bell
                        // The bell often occurs when characters can't be processed
                        #[cfg(debug_assertions)]
                        {
                            eprintln!("Could not convert event to command: {_err:?}");
                        }
                    }
                }
            }
        }
    }
    // Remove the else block or make it silent to prevent unnecessary processing
}
    
    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();
        let _ = Terminal::move_caret_to(self.view.caret_position());

        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}
