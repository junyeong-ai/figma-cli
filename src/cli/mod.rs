//! CLI interface

pub mod args;
pub mod commands;
pub mod output;

pub use args::{Cli, Commands, OutputFormat};
pub use commands::{handle_auth, handle_config, handle_extract, handle_images, handle_inspect};
pub use output::format_output;
