use std::io::Stdout;

use crossterm::event::Event;

use crate::app::Data;

pub trait Menu<T: Data> {
    /// Displays the current state of the app on the standard output, using the app's data and
    /// menu.
    ///
    /// Will be called each time an event happens.
    /// That also means the display will not update without an event.
    fn display(&self, data: &T, stdout: &mut Stdout) -> std::io::Result<()>;
    /// Changes the app's data and menu depending on some input.
    ///
    /// Will be called with every event happening within the app.
    fn handle_event(&mut self, data: &mut T, event: Event) -> Option<Box<dyn Menu<T>>>;
}
