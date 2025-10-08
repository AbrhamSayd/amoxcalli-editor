use std::io::Error;

use super::terminal::Size;

pub trait UIComponent {
    // Marks this UI component as in need of redrawing (or not)
    fn set_requires_redraw(&mut self, requires_redraw: bool);

    // Determine if this component needs to be redrawn
    fn requires_redraw(&self) -> bool;

    // Update the size and redraws
    fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.set_requires_redraw(true);
    }

    //Updates the size. Must be implemented by the component.
    fn set_size(&mut self, size: Size);

    // Draw this component if it's visible and in need of redrawing
    fn render(&mut self, origin_y: usize){
        if self.requires_redraw() {
            match self.draw(origin_y) {
                Ok(()) => self.set_requires_redraw(false),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Error rendering component: {err:?}" );
                    }
                }, //TODO: Handle error
            }
        }
    }
    // Draw this component at the given y-origin. Must be implemented by the component.
    fn draw(&mut self, origin_y: usize) -> Result<(), Error>;



}