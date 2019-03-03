use std::env::args;

fn main() {
    for name in args() {
        let first_letter = &name[0..1];
        match first_letter {
            "w" | "W" => println!("Hello {}", name),
            _ => {}
        }
    }
}
