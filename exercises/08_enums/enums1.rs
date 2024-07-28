#[derive(Debug)]
enum Message {
    Resize,
    Quit,
    Echo,
    Move,
    ChangeColor
    // TODO: Define a few types of messages as used below.
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
