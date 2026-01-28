use crossterm::event::{Event, KeyEvent, KeyEventKind, read};
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};
mod command;
mod commandbar;
mod commandparser;
mod documentstatus;
mod line;
mod messagebar;
mod mode;
mod position;
mod size;
mod statusbar;
mod terminal;
mod uicomponent;
mod view;
use line::Line;
use documentstatus::DocumentStatus;

use self::command::{
    Command::{self, Edit, Move, System},
    Edit::InsertNewLine,
    System::{Dismiss, Resize, ShowCommandBar},
};
use messagebar::MessageBar;
use mode::Mode;
use position::Position;
use size::Size;
use statusbar::StatusBar;
use terminal::Terminal;
use uicomponent::UIComponent;
use view::View;
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
    message_bar: MessageBar,
    command_bar: Option<commandbar::CommandBar>,
    terminal_size: Size,
    title: String,
    quit_times: u8,
    mode: Mode,
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
            if editor.view.load(file_name).is_err() {
                editor
                    .message_bar
                    .update_message(&format!("ERR: Could not open file: {file_name}"));
            }
        }
        editor.refresh_status();
        editor.status_bar.set_requires_redraw(true);
        Ok(editor)
    }

    pub fn resize(&mut self, size: Size) {
        self.terminal_size = size;
        self.view.resize(Size {
            height: size.height.saturating_sub(2),
            width: size.width,
        });

        self.status_bar.resize(Size {
            height: 1,
            width: size.width,
        });

        self.message_bar.resize(Size {
            height: 1,
            width: size.width,
        });

        if let Some(command_bar) = &mut self.command_bar {
            command_bar.resize(Size {
                height: 1,
                width: size.width,
            });
        }
    }

    pub fn refresh_status(&mut self) {
        let status = self.view.get_status();
        let mode_str = match self.mode {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
        };

        let title = format!("{} - {NAME} [{}]", status.file_name, mode_str);
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

        #[cfg(debug_assertions)]
        if let Event::Key(key_event) = &event {
            self.message_bar.update_message(&format!(
                "Key: {:?}, Mods: {:?}, Kind: {:?}, State: {:?}",
                key_event.code, key_event.modifiers, key_event.kind, key_event.state
            ));
        }

        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if should_process {
            if let Ok(command) = Command::try_from(event) {
                self.process_command(command);
            }
        }
    }

    fn process_command(&mut self, command: Command) {
        match command {
            System(Resize(size)) => self.resize(size),
            _ => self.reset_quit_times(),
        }

        match command {
            System( Resize(_)) => {}
            System(Dismiss) => {
                if self.command_bar.is_some() {
                    self.dismiss_prompt();
                    self.message_bar.update_message("Command cancelled.");
                } else if self.mode.is_insert() {
                    // ESC exits insert mode
                    self.mode = Mode::Normal;
                    self.message_bar.update_message("");
                    self.refresh_status();
                }
            }
            System(ShowCommandBar) => {
                self.show_prompt();
            }
            Edit(edit_command) => {
                if let Some(command_bar) = &mut self.command_bar {
                    if matches!(edit_command, InsertNewLine) {
                        let command_input = command_bar.value();
                        self.dismiss_prompt();
                        self.execute_command(&command_input);
                    } else {
                        command_bar.handle_edit_command(edit_command);
                    }
                } else if self.mode.is_insert() {
                    // Only allow editing in Insert mode
                    self.view.handle_edit_command(edit_command);
                } else if self.mode.is_normal() {
                    // In Normal mode, 'i' or 'I' enters Insert mode
                    if let command::Edit::Insert(ch) = edit_command {
                        match ch {
                            '\u{1b}' => { /* Ignore ESC in normal mode */},
                            'i' => {
                                self.mode = Mode::Insert;
                                self.message_bar.update_message("-- INSERT --");
                                self.refresh_status();
                            }
                            'I' => {
                                self.view.handle_move_command(command::Move::StartOfLine);
                                self.mode = Mode::Insert;
                                self.message_bar.update_message("-- INSERT --");
                                self.refresh_status();
                            },
                        _ => {},
                            
                        }
                    }
                }
            }
            Move(move_command) => {
                if self.command_bar.is_none() {
                    self.view.handle_move_command(move_command);
                }
            }
        }
    }
    fn dismiss_prompt(&mut self) {
        self.command_bar = None;
        self.message_bar.set_requires_redraw(true);
    }

    fn show_prompt(&mut self) {
        let mut command_bar = commandbar::CommandBar::default();
        command_bar.set_prompt(":");
        command_bar.resize(Size {
            height: 1,
            width: self.terminal_size.width,
        });
        command_bar.set_requires_redraw(true);
        self.command_bar = Some(command_bar);
    }

    fn save(&mut self, file_name: Option<&str>) -> Result<(), std::io::Error> {
        let result = if let Some(name) = file_name {
            self.view.save_as(name)
        } else {
            self.view.save()
        };

        match &result {
            Ok(_) => {
                self.message_bar.update_message("File saved successfully.");
                self.refresh_status(); // Refresh to update modified status
            }
            Err(_) => {
                self.message_bar.update_message("Error writing file!");
            }
        }
        result
    }

    fn reset_quit_times(&mut self) {
        if self.quit_times > 0 {
            self.quit_times = 0;
            self.message_bar.update_message("");
        }
    }

    fn refresh_screen(&mut self) {
        if self.terminal_size.height == 0 || self.terminal_size.width == 0 {
            return;
        }
        let bottom_bar_row = self.terminal_size.height.saturating_sub(1);
        let _ = Terminal::hide_caret();

        if let Some(command_bar) = &mut self.command_bar {
            command_bar.render(bottom_bar_row);
        } else {
            self.message_bar.render(bottom_bar_row);
        }

        if self.terminal_size.height > 1 {
            self.status_bar
                .render(self.terminal_size.height.saturating_sub(2));
        }

        if self.terminal_size.height > 2 {
            self.view.render(0);
        }

        let new_carret_position = if let Some(command_bar) = &self.command_bar {
            Position {
                row: bottom_bar_row,
                col: command_bar.caret_position_col(),
            }
        } else {
            self.view.caret_position()
        };
        let _ = Terminal::move_caret_to(new_carret_position);
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }

    fn execute_command(&mut self, input: &str) {
        use commandparser::ParsedCommand;

        let command = ParsedCommand::parse(input);

        match command {
            ParsedCommand::Write => {
                if self.view.is_file_loaded() {
                    let _ = self.save(None);
                } else {
                    self.message_bar
                        .update_message("No file name. Use :w <filename>");
                }
            }
            ParsedCommand::WriteAs(filename) => {
                let _ = self.save(Some(&filename));
            }
            ParsedCommand::Quit => {
                if self.view.get_status().is_modified {
                    self.message_bar
                        .update_message("No write since last change. Use :q! to force quit.");
                } else {
                    self.should_quit = true;
                }
            }
            ParsedCommand::ForceQuit => {
                self.should_quit = true;
            }
            ParsedCommand::WriteQuit => {
                if self.view.is_file_loaded() {
                    if self.save(None).is_ok() {
                        self.should_quit = true;
                    }
                } else {
                    self.message_bar
                        .update_message("No file name. Use :wq <filename>");
                }
            }
            ParsedCommand::WriteAsAndQuit(filename) => {
                if self.save(Some(&filename)).is_ok() {
                    self.should_quit = true;
                }
            }
            ParsedCommand::Unknown(cmd) => {
                self.message_bar
                    .update_message(&format!("Unknown command: {}", cmd));
            }
            ParsedCommand::Help =>{
                let help_message = "Commands: :w (write), :w <filename> (write as), :q (quit), :q! (force quit), :wq (write and quit), :wq <filename> (write as and quit), :help (this message)";
                self.message_bar.update_message(help_message);
            }
        }
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
