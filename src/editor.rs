use crossterm::event::{read, Event, KeyEvent, KeyEventKind};
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};
mod documentstatus;
mod fileinfo;
mod statusbar;
mod terminal;
mod view;
mod commands;
mod messagebar;
mod uicomponent;
use uicomponent::UIComponent;
use documentstatus::DocumentStatus;
use commands::EditorCommand;
use statusbar::StatusBar;
use terminal::Terminal;
use view::View;

use self::{messagebar::MessageBar, terminal::Size};

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");


#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
    message_bar: MessageBar,
    terminal_size: Size,
    title: String,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;

        let mut editor = Self::default();
        let size = Terminal::size().unwrap_or_default();
        editor.resize(size);

        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            editor.view.load(file_name);
        }

        editor
        .message_bar
        .update_message("HELP: Ctrl-S = save | Ctrl-Q = quit | Ctrl-F = find".to_string());

        editor.refresh_status();
        Ok(editor)
    }

    pub fn resize(&mut self, size: Size) {
        self.terminal_size = size;
        self.view.resize(Size {
            height: size.height.saturating_sub(2),
            width: size.width,
        });

        self.message_bar.resize(Size {
            height: 1,
            width: size.width,
        });

        self.status_bar.resize(Size {
            height: 1,
            width: size.width,
        });
    }

    pub fn refresh_status(&mut self) {
        let status = self.view.get_status();
        
        let title = format!("{} - {NAME}", status.file_name);
        self.status_bar.update_status(status);
        if title != self.title && matches!(Terminal::set_title(&title), Ok(())) {
            self.title = title;
        }
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

            let status = self.view.get_status();
            self.status_bar.update_status(status);
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
                        }else if let EditorCommand::Resize(size) = command{
                            self.resize(size);
                        } else {
                            self.view.handle_command(command);
                        }
                    }
                    Err(_err) => {
                        // Silently ignore unrecognized commands to prevent terminal bell
                        // The bell often occurs when characters can't be processed
                        #[cfg(debug_assertions)]
                        {
                           
                        }
                    }
                }
            }
        }
    }
    // Remove the else block or make it silent to prevent unnecessary processing
}
    
    fn refresh_screen(&mut self) {
        if self.terminal_size.height == 0 || self.terminal_size.width == 0 {
            return;
        }
        let _ = Terminal::hide_caret();
        self.message_bar.render(self.terminal_size.height.saturating_sub(1));

        if self.terminal_size.height > 1 {
            self.status_bar.render(self.terminal_size.height.saturating_sub(2));
        }

        if self.terminal_size.height > 2 {
            self.view.render(0);
        }
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
