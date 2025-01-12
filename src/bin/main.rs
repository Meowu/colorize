use colorize::Color;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: colorize \"<hex | rgb(a) | hsl>\"");
        std::process::exit(1);
    }
    let color_str = args.get(1).unwrap();
    if let Ok(color) = Color::parse_color(color_str) {
        println!("RGB: {}", color.to_rgb_string());
        println!("RGBA: {}", color.to_rgba_string());
        println!("HEX: {}", color.to_hex_string());
        println!("HSL: {}", color.to_hsl_string());
    } else {
        eprintln!("Invalid color: {}", color_str);
        std::process::exit(1);
    }
}
