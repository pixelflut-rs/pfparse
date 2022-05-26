use std::num::ParseIntError;

use nom::bytes::complete::take;
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::IResult;

pub const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};

#[derive(PartialEq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (_, (r, g, b)) = match colors_parser(input) {
            Ok(r) => r,
            Err(err) => return Err(err),
        };

        return Ok(("", Color { r, g, b, a: 255 }));
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        let b: [u8; 4] = [color.a, color.r, color.g, color.b];
        u32::from_be_bytes(b)
    }
}

impl From<&Color> for u32 {
    fn from(color: &Color) -> Self {
        let b: [u8; 4] = [color.a, color.r, color.g, color.b];
        u32::from_be_bytes(b)
    }
}

impl From<u32> for Color {
    fn from(color: u32) -> Self {
        let b = color.to_ne_bytes();
        Color {
            r: b[2],
            g: b[1],
            b: b[0],
            a: 255,
        }
    }
}

impl From<&u32> for Color {
    fn from(color: &u32) -> Self {
        let b = color.to_ne_bytes();
        Color {
            r: b[2],
            g: b[1],
            b: b[0],
            a: 255,
        }
    }
}

fn colors_parser(input: &str) -> IResult<&str, (u8, u8, u8)> {
    tuple((
        map_res(take(2usize), to_u8),
        map_res(take(2usize), to_u8),
        map_res(take(2usize), to_u8),
    ))(input)
}

fn to_u8(input: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(input, 16)
}
