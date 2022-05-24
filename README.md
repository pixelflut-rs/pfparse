# pfparse

Parse Pixelflut commands:

- `PX`: Request and set command
- `SIZE`: Request canvas size
- `HELP`: Request help

## Usage

```rust
let r = match pfparse::parse("PX 10 10 FFFFFF") {
    Ok(r) => r,
    Err(err) => panic!("{}", err),
};

r = (
    "",
    pfparse::Command::Pixel {
        is_req: false,
        x: 10,
        y: 10,
        c: 0
    }
)
```