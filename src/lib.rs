pub mod color;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::combinator::opt;
use nom::combinator::value;
use nom::combinator::verify;
use nom::sequence::separated_pair;
use nom::sequence::tuple;
use nom::IResult;

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

pub fn parse(input: &str) -> IResult<&str, Command> {
    // First we match the command. An unknown command returns an error
    let (rest, cmd) = match alt((
        value(0, tuple((tag("PX"), tag(" ")))),
        value(1, tag("SIZE")),
        value(2, tag("HELP")),
    ))(input)
    {
        Err(err) => return Err(err),
        Ok(v) => v,
    };

    // Immediatly return the SIZE and HELP commands, fall through for PX command
    match cmd {
        1 => return Ok(("", Command::Size)),
        2 => return Ok(("", Command::Help)),
        _ => {}
    };

    // Further parse the PX command. This can be either a request to get the color at <x, y> or to set a color at <x, y>
    let (_, ((x, y), c)) = match tuple((coord_parser, color_parser))(rest) {
        Err(err) => return Err(err),
        Ok(r) => r,
    };

    // Parse color
    let (_, color) = match color::Color::parse(c.1) {
        Err(err) => return Err(err),
        Ok(c) => c,
    };

    return Ok((
        "",
        Command::Pixel {
            is_req: c.0,
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            c: color,
        },
    ));
}

fn color_parser(input: &str) -> IResult<&str, (bool, &str)> {
    let (rest, color) = match opt(tuple((tag(" "), take(6usize))))(input) {
        Err(err) => return Err(err),
        Ok(r) => r,
    };

    // Check if we found other than None. If not it is a request
    let (is_req, color) = match color {
        Some(color) => {
            if color.1.len() != 6 {
                return Err(nom::Err::Failure(nom::error::Error {
                    code: nom::error::ErrorKind::Verify,
                    input,
                }));
            }
            (false, color.1)
        }
        None => (true, ""),
    };

    Ok((rest, (is_req, color)))
}

fn coord_parser(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(digit1, char(' '), digit1)(input)
}
