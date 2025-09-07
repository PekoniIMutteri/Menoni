#![deny(missing_docs)]

//! CLI Application helper.
//!
//! To help doing little CLI applications, this crate defines `App`, `Data` and `Menu`. To make an
//! `App`, you define all the `Menu`s and how they interact with the saved `Data`.
//!
//! The following example shows a minimal app able to switch between 2 menus.

mod app;

pub use app::App;
pub use app::Data;
pub use app::menu::InputFunction;
pub use app::menu::Menu;
pub use app::menu::keyboard_inputs;
