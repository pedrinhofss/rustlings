// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

#[derive(Debug)]
enum Message {
    Quit = 314,
    Echo = 315,
    Move = 316,
    ChangeColor = 317
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
