use thiserror::Error;

#[derive(Debug, Default)]
pub enum CommandParseState {
    #[default]
    PreCheck,
    CommandBytes,
    Selection,
    Pixel,
    Offset,
}

#[derive(Debug, Error)]
pub enum CommandParseError {
    #[error("invalid byte")]
    InvalidByte,

    #[error("buffer too short")]
    BufTooShort,

    #[error("unknown/unsupported command")]
    UnknownCommand,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Help,
    Size,
    PixelGet { x: usize, y: usize },
    PixelSet { c: u32, x: usize, y: usize },
    Offset { x: usize, y: usize },
}

pub async fn parse_command(buffer: &[u8]) -> Result<Command, CommandParseError> {
    let mut state = CommandParseState::default();
    let mut i = 0;

    loop {
        state = match state {
            CommandParseState::PreCheck => {
                if buffer.len() < 4 {
                    return Err(CommandParseError::BufTooShort);
                }

                CommandParseState::CommandBytes
            }
            CommandParseState::CommandBytes => {
                match buffer[i] {
                    b'P' | b'X' | b'S' | b'I' | b'Z' | b'E' | b'H' | b'L' | b'O' | b'F' | b'T' => {
                        // We encountered a valid byte. This means we increase
                        // our index by one and continue the loop without
                        // updating the parser state.
                        i += 1;
                        continue;
                    }
                    b' ' | b'\n' => {
                        // We encountered a newline. This
                        // indicates the command is finished and we now can
                        // either select the command in case of SIZE and HELP
                        // or continue parsing bytes in case of PX and OFFSET.
                        CommandParseState::Selection
                    }
                    _ => return Err(CommandParseError::InvalidByte),
                }
            }
            CommandParseState::Selection => {
                // Select proper command
                match buffer.split_at(i) {
                    (&[b'P', b'X'], _) => CommandParseState::Pixel,
                    (&[b'O', b'F', b'F', b'S', b'E', b'T'], _) => CommandParseState::Offset,
                    (&[b'H', b'E', b'L', b'P'], _) => return Ok(Command::Help),
                    (&[b'S', b'I', b'Z', b'E'], _) => return Ok(Command::Size),
                    _ => return Err(CommandParseError::UnknownCommand),
                }
            }
            CommandParseState::Pixel => {
                // Look for the X coordinate. We do this by looping over the
                // bytes until we encounter a whitespace, which separates the
                // X from the Y coordinate.
                i += 1;

                let mut x = 0;
                loop {
                    if buffer[i] == b' ' {
                        i += 1;
                        break;
                    }
                    x = 10 * x + (buffer[i] - b'0') as usize;
                    i += 1;
                }

                // Let's do the same as above for the Y coordinate.
                let mut y = 0;
                loop {
                    if buffer[i] == b' ' || buffer[i] == b'\n' {
                        break;
                    }
                    y = 10 * y + (buffer[i] - b'0') as usize;
                    i += 1;
                }

                // Exit early if user requested pixel at (X, Y)
                if buffer[i] == b'\n' {
                    return Ok(Command::PixelGet { x, y });
                }

                // Skip whitespace
                i += 1;

                let str = unsafe { std::str::from_utf8_unchecked(&buffer[i..i + 6]) };
                let color = u32::from_str_radix(str, 16).unwrap(); // TODO handle this

                return Ok(Command::PixelSet { c: color, x, y });
            }
            CommandParseState::Offset => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{parse_command, Command};

    #[tokio::test]
    async fn parse_help_command() {
        let input = "HELP\n";
        let input = input.as_bytes();

        match parse_command(input).await {
            Ok(cmd) => assert_eq!(cmd, Command::Help),
            Err(err) => panic!("{err:?}"),
        }
    }

    #[tokio::test]
    async fn parse_size_command() {
        let input = "SIZE\n";
        let input = input.as_bytes();

        match parse_command(input).await {
            Ok(cmd) => assert_eq!(cmd, Command::Size),
            Err(err) => panic!("{err:?}"),
        }
    }

    #[tokio::test]
    async fn parse_pixel_get_command() {
        let input = "PX 10 10\n";
        let input = input.as_bytes();

        match parse_command(input).await {
            Ok(cmd) => assert_eq!(cmd, Command::PixelGet { x: 10, y: 10 }),
            Err(err) => panic!("{err:?}"),
        }
    }

    #[tokio::test]
    async fn parse_pixel_set_command() {
        let input = "PX 10 10 000000\n";
        let input = input.as_bytes();

        match parse_command(input).await {
            Ok(cmd) => assert_eq!(cmd, Command::PixelSet { x: 10, y: 10, c: 0 }),
            Err(err) => panic!("{err:?}"),
        }
    }
}
