pub mod data;
pub mod menu;

use std::io::{self, stdout};

use crossterm::{
    event::read,
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use menu::Menu;

/// A whole program.
///
/// An app contains some data and some menus to open.
///
/// See [examples](run)
pub struct App<T: Data + Default> {
    data: T,
    menu: Box<dyn Menu<T>>,
    name: String,
}

/// A trait the App's data should implement to save and load from files, and stop the app.
///
/// This is the data an app will save between menus, and even between executions of the program.
pub trait Data: std::marker::Sized {
    /// Wether the app is or is not running.
    fn is_running(&self) -> bool;
    /// Wether the app will automatically display the name of the app on top of the screen.
    fn should_display_name(&self) -> bool;
    /// Gets a new Data from the bytes saved using `self.save()`.
    fn from_saved(bytes: &[u8]) -> io::Result<Self>;
    /// Returns the bytes to save between executions.
    fn save(&self) -> Vec<u8>;
}

impl<T: Data + Default> App<T> {
    /// Initializes a new app with the given data and menu.
    ///
    /// For usage examples, see [run]
    pub fn new(data: T, initial_menu: Box<dyn Menu<T>>, name: &str) -> App<T> {
        App {
            data,
            menu: initial_menu,
            name: name.to_string(),
        }
    }
    /// Starts the app, looping until told otherwise.
    ///
    /// Enters the terminal's alternate screen, enables raw mode, and starts a loop.
    /// To stop the app, the app's `data.running()` should return false.
    /// After stopping, leaves alternate screen and disables raw mode.
    ///
    /// # Example
    ///
    /// ```
    /// let mut app = App::new(MyData, Box::new(MyMainMenu));
    /// app.run()?;
    /// ```
    pub fn run(&mut self) -> std::io::Result<()> {
        let mut stdout = stdout();
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen)?;
        self.menu.display(&self.data, &mut stdout)?;
        while self.data.is_running() {
            let event = read()?;
            let new_menu = self.menu.handle_event(&mut self.data, event);
            if let Some(menu) = new_menu {
                self.menu = menu;
            }
            self.menu.display(&self.data, &mut stdout)?;
        }
        execute!(stdout, LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
}
