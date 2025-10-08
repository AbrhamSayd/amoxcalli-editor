use std::{io::Error, time::{Duration, Instant}};

use super:: {
    terminal:: {Size, Terminal},
    uicomponent::UIComponent,
};

const DEFAULT_DURATION: Duration = Duration::from_secs(5);

struct Message {
    text: String,
    time: Instant,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            text: String::new(),
            time: Instant::now(),   
        }
    }
}

impl Message {
    fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.time) > DEFAULT_DURATION
    }
}
#[derive(Default)]
pub struct MessageBar {
    current_message: Message,
    requires_redraw: bool,
    cleared_after_expiry: bool,
}

impl MessageBar{
    pub fn update_message(&mut self, new_message: &str) {
        self.current_message = Message {
            text: new_message.to_string(),
            time: Instant::now(),
        };
        self.cleared_after_expiry = false;
        self.requires_redraw = true;
    }
}

impl UIComponent for MessageBar {
    fn set_requires_redraw(&mut self, value: bool) {
        self.requires_redraw = value;
    }

    fn requires_redraw(&self) -> bool {
        (!self.cleared_after_expiry && self.current_message.is_expired()) || self.requires_redraw
    }

    fn set_size(&mut self, _: Size) {
       
    }

    fn draw(&mut self, origin: usize) -> Result<(), Error>  {
        if self.current_message.is_expired() {
            self.cleared_after_expiry = true;
        }
        let message = if self.current_message.is_expired(){
            ""
        } else {
            &self.current_message.text
        };
        Terminal::print_row(origin, message)
    }
}
