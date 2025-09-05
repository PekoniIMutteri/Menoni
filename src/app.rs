use std::io::stdout;

use crossterm::{
    event::read,
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use crate::Menu;

pub struct App<T: Data> {
    pub data: T,
    pub menu: Box<dyn Menu<T>>,
}

pub trait Data {
    /// Wether the app is or is not running.
    fn running(&self) -> bool;
}

impl<T: Data> App<T> {
    pub fn new(data: T, initial_menu: Box<dyn Menu<T>>) -> App<T> {
        App {
            data,
            menu: initial_menu,
        }
    }
    pub fn data(&self) -> &T {
        &self.data
    }
    pub fn menu(&self) -> &dyn Menu<T> {
        &*self.menu
    }
    pub fn main_loop(&mut self) -> std::io::Result<()> {
        let mut stdout = stdout();
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen)?;
        self.menu.display(&self.data, &mut stdout)?;
        while self.data.running() {
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
