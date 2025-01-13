# colorize

`colorize` is a simple and efficient command-line color conversion tool. It supports conversion between multiple color formats including RGB, RGBA, HEX, and HSL.

## Features

- Supports parsing colors from strings in RGB, RGBA, HEX, and HSL formats.
- Outputs the parsed color in RGB, RGBA, HEX, and HSL formats.

## Usage

First, make sure you have Rust development environment installed.

1. Clone the repository:

   ```bash
   git clone git@github.com:Meowu/colorize.git
   cd colorize
   ```

2. Run the project:

   You can run the program using:

   ```bash
   cargo run -- "<color>"
   ```

   Where `<color>` can be in one of the following formats:

   - HEX: `#RRGGBB` or `#RRGGBBAA`
   - RGB: `rgb(r, g, b)`, e.g., `rgb(255, 0, 0)`
   - RGBA: `rgba(r, g, b, a)`, e.g., `rgba(255, 0, 0, 0.5)`
   - HSL: `hsl(h, s%, l%)`, e.g., `hsl(120, 100%, 50%)`

3. Example:

   ```bash
   cargo run -- "#FF5733"
   ```

## Contributions

Bug reports and feature requests are welcome. If you want to contribute code, please fork the project and send a pull request.
