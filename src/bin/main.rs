use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let color = args.get(1);
    if let Some(color) = color {
        println!("color: {}", color);
    } else {
        println!("Usage: colorize \"<hex | rgb(a) | hsl>\"");
        std::process::exit(1);
    }
}
