#![allow(unused)]

use rust_template::config;
use rust_template::prelude::*;

fn main() -> Result<()> {
    rust_template::run(config::Config::new())?;

    Ok(())
}
