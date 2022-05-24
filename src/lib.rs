use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::combinator::opt;
use nom::combinator::value;
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
        c: u32,
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
        Ok(v) => v,
        Err(err) => return Err(err),
    };

    // Immediatly return the SIZE and HELP commands, fall through for PX command
    match cmd {
        1 => return Ok(("", Command::Size)),
        2 => return Ok(("", Command::Help)),
        _ => {}
    };

    // Further parse the PX command. This can be either a request to get the color at <x, y> or to set a color at <x, y>
    let (_, ((x, y), c)) = match tuple((coord_parser, opt(tuple((tag(" "), color_parser)))))(rest) {
        Ok(r) => r,
        Err(err) => return Err(err),
    };

    println!("{:?}", c);
    // Match if request or not
    let (is_req, c) = match c {
        Some(c) => (false, c.1.parse().unwrap()),
        None => (true, 0),
    };

    return Ok((
        "",
        Command::Pixel {
            is_req: is_req,
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            c: c,
        },
    ));
}

fn color_parser(input: &str) -> IResult<&str, &str> {
    take(6usize)(input)
}

fn coord_parser(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(digit1, char(' '), digit1)(input)
}
