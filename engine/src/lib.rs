//! Game engine sitting atop the core libraries.
#[macro_use]
extern crate amethyst_config;
extern crate amethyst_context;
extern crate amethyst_ecs;
extern crate amethyst_processors;
extern crate amethyst_renderer;

mod app;
mod state;
mod config;

pub use self::app::{Application, ApplicationBuilder};
pub use self::state::{State, StateMachine, Trans};
pub use self::config::Config;
