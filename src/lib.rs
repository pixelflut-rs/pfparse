pub mod color;
pub mod error;
mod impls;

use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Command {
    Help,
    Size,
    Pixel {
        is_req: bool,
        x: usize,
        y: usize,
        c: color::Color,
    },
}

impl FromStr for Command {}
