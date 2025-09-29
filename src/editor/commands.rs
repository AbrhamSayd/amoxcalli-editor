use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::convert::TryFrom;


use super::terminal::Size;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
}

pub enum EditorCommand {
    Resize(Size),
    Move(Direction),
    Quit,

}

impl TryFrom<Event> for EditorCommand {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent { code, modifiers, ..}) =>
            match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(EditorCommand::Quit),
                (KeyCode::Up, KeyModifiers::NONE) => Ok(EditorCommand::Move(Direction::Up)),
                (KeyCode::Down, KeyModifiers::NONE) => Ok(EditorCommand::Move(Direction::Down)),
                (KeyCode::Left, KeyModifiers::NONE) => Ok(EditorCommand::Move(Direction::Left)),
                (KeyCode::Right, KeyModifiers::NONE) => Ok(EditorCommand::Move(Direction::Right)),
                (KeyCode::PageUp, KeyModifiers::NONE) => Ok(EditorCommand::Move(Direction::PageUp)),
                (KeyCode::PageDown, KeyModifiers::NONE) => Ok(EditorCommand::Move(Direction::PageDown)),
                (KeyCode::Home, KeyModifiers::NONE) => Ok(EditorCommand::Move(Direction::Home)),
                (KeyCode::End, KeyModifiers::NONE) => Ok(EditorCommand::Move(Direction::End)),
                _ => Err(format!("Unrecognized key event: {event:?}")),
            },
            Event::Resize(width_u16, height_u16) => {
                #[allow(clippy::as_conversions)]
                let width = width_u16 as usize;
                #[allow(clippy::as_conversions)]
                let height = height_u16 as usize;
                Ok(EditorCommand::Resize(Size { height, width }))
            }
            _ => Err(format!("Unrecognized event: {event:?}")),
        }
    }

}