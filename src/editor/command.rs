use crossterm::event::{
    Event,
    KeyCode::{
        self, Backspace, Char, Delete, Down, End, Enter, Home, Left, PageDown, PageUp, Right, Tab,
        Up,
    },
    KeyEvent, KeyModifiers,
};

use std::convert::TryFrom;
use super::Size;

#[derive(Clone, Copy)]
pub enum Move {
    PageUp,
    PageDown,
    StartOfLine,
    EndOfLine,
    Up,
    Left,
    Right,
    Down,
}

impl TryFrom<KeyEvent> for Move {
    type Error = String;
    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::NONE {
            match code {
                Up => Ok(Self::Up),
                Down => Ok(Self::Down),
                Left => Ok(Self::Left),
                Right => Ok(Self::Right),
                PageUp => Ok(Self::PageUp),
                PageDown => Ok(Self::PageDown),
                Home => Ok(Self::StartOfLine),
                End => Ok(Self::EndOfLine),
                _ => Err(format!("Unrecognized move key event: {code:?}")),
            }
        } else {
            Err(format!(
                "Unrecognized move key event with modifiers: {modifiers:?}",
            ))
        }
    }
}

#[derive(Clone, Copy)]
pub enum Edit {
    Insert(char),
    InsertNewLine,
    Delete,
    DeleteBackward,
}

impl TryFrom<KeyEvent> for Edit {
    type Error = String;
    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        match (event.code, event.modifiers) {
            (Char(character), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                Ok(Self::Insert(character))
            }
            (Tab, KeyModifiers::NONE) => Ok(Self::Insert('\t')),
            (Enter, KeyModifiers::NONE) => Ok(Self::InsertNewLine),
            (Backspace, KeyModifiers::NONE) => Ok(Self::DeleteBackward),
            (Delete, KeyModifiers::NONE) => Ok(Self::Delete),
            _ => Err(format!(
                "Unsupported key code {:?} with modifiers {:?}",
                event.code, event.modifiers
            )),
        }
    }
}

#[derive(Clone, Copy)]
pub enum System {
    Resize(Size),
    Dismiss,
    ShowCommandBar,
}

impl TryFrom<KeyEvent> for System {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;
        
        if modifiers == KeyModifiers::NONE {
            match code {
                KeyCode::Esc => Ok(Self::Dismiss),
                Char(':') => Ok(Self::ShowCommandBar),
                _ => Err(format!("Unrecognized key: {code:?}")),
            }
        } else if modifiers == KeyModifiers::CONTROL {
            match code {
                _ => Err(format!("Unrecognized CONTROL+{code:?} combination")),
            }
        } else {
            Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}",
            ))
        }
    }
}

#[derive(Clone, Copy)]
pub enum Command {
    Move(Move),
    Edit(Edit),
    System(System),
}

#[allow(clippy::as_conversions)]
impl TryFrom<Event> for Command {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(key_event) => System::try_from(key_event)  // Check System FIRST
                .map(Command::System)
                .or_else(|_| Move::try_from(key_event).map(Command::Move))
                .or_else(|_| Edit::try_from(key_event).map(Command::Edit))  // Edit LAST
                .map_err(|_err| format!("Event not supported: {key_event:?}")),
            Event::Resize(width_u16, height_u16) => Ok(Self::System(System::Resize(Size {
                height: height_u16 as usize,
                width: width_u16 as usize,
            }))),
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}
