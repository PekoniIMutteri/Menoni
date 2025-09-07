use std::io::Stdout;

use crossterm::event::{Event, KeyCode};

use crate::app::Data;

/// A trait to use a structure as the app's menu.
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
    /// If the menu has a title, it will automatically be displayed on the middle of the top of the
    /// screen.
    fn title(&self) -> Option<&str>;
}

/// The type of a function to be used in input handling (with `keyboard_inputs(...)`).
pub type InputFunction<M, D> = fn(&mut M, &mut D) -> Option<Box<dyn Menu<D>>>;

/// test
///
/// ## Common Mishaps:
///
/// When creating a vector with different functions for actions, the compilator will complain that
/// they aren't all the same type. You have to tell it that you don't want functions with the same
/// name, but just functions with the `InputFunction` type, done by setting manually the vec's
/// type.
pub fn keyboard_inputs<M: Menu<D>, D: Data>(
    menu: &mut M,
    data: &mut D,
    event: Event,
    actions: Vec<(char, InputFunction<M, D>)>,
) -> Option<Box<dyn Menu<D>>> {
    if let Event::Key(key) = event
        && key.is_press()
    {
        for (char, func) in actions {
            if key.code == KeyCode::Char(char) {
                return (func)(menu, data);
            }
        }
    }
    None
}
