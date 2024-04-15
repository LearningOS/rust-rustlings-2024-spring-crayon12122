// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.



#[derive(Debug)]
//任何类型的成员都可放入枚举成员中
enum Message {
    // TODO: define the different variants used below
    //move包含一个匿名结构体
    Move {x: i32, y: i32},
    //echo包含一个String字符串
    Echo(String),
    //ChangeColor包含3个i32
    ChangeColor(i32, i32, i32),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
