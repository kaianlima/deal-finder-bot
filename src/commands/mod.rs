pub mod scraper;
pub mod ping;

pub use anyhow::{Error, Result};

use crate::structs::Command;

pub fn commands() -> Vec<Command> {
    scraper::commands().into_iter()
        .chain(ping::commands())
        .collect()
}