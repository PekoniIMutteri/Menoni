use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode},
    execute,
    style::Print,
    terminal::{Clear, ClearType, size},
};
use menoni::*;

fn main() {
    println!("Hello World !");
    let mut app = App::new(MyData::default(), Box::new(Main));
    app.main_loop().unwrap();
    println!("Goodbye !");
}

struct MyData {
    running: bool,
    message: String,
}

impl Default for MyData {
    fn default() -> Self {
        MyData {
            running: true,
            message: "Secret stone ? Demon King ?!".to_string(),
        }
    }
}

impl Data for MyData {
    fn running(&self) -> bool {
        self.running
    }
}

struct Main;

impl Menu<MyData> for Main {
    fn display(&self, _data: &MyData, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        execute!(
            stdout,
            Clear(ClearType::All),
            MoveTo(0, 0),
            Print("MAIN MENU"),
            MoveTo(0, size()?.1 - 1),
        )
    }
    fn handle_event(
        &mut self,
        data: &mut MyData,
        event: crossterm::event::Event,
    ) -> Option<Box<dyn Menu<MyData>>> {
        keyboard_inputs!(self, data, event, 'q', quit, 'm', switch_to_message);
        None
    }
}

struct Message;

impl Menu<MyData> for Message {
    fn display(&self, data: &MyData, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        execute!(
            stdout,
            Clear(ClearType::All),
            MoveTo(0, 0),
            Print("MESSAGE"),
            MoveTo(0, 2),
            Print(&data.message),
            MoveTo(0, size()?.1 - 1),
        )
    }
    fn handle_event(&mut self, data: &mut MyData, event: Event) -> Option<Box<dyn Menu<MyData>>> {
        keyboard_inputs!(self, data, event, 'm', switch_to_main, 'q', quit);
        None
    }
}

fn quit<T: Menu<MyData>>(_menu: &mut T, data: &mut MyData) -> Option<Box<dyn Menu<MyData>>> {
    data.running = false;
    None
}

fn switch_to_main<T: Menu<MyData>>(
    _menu: &mut T,
    _data: &mut MyData,
) -> Option<Box<dyn Menu<MyData>>> {
    Some(Box::new(Main))
}

fn switch_to_message<T: Menu<MyData>>(
    _menu: &mut T,
    _data: &mut MyData,
) -> Option<Box<dyn Menu<MyData>>> {
    Some(Box::new(Message))
}
