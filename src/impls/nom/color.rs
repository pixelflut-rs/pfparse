use std::num::ParseIntError;

use nom::{bytes::complete::take, combinator::map_res, sequence::tuple, IResult};

use crate::color::Color;

pub fn parse(input: &str) -> IResult<&str, Color> {
    let (_, (r, g, b)) = match colors_parser(input) {
        Ok(r) => r,
        Err(err) => return Err(err),
    };

    return Ok(("", Color { r, g, b, a: 255 }));
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
