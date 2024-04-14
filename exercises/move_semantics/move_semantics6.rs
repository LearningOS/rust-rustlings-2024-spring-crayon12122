// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let mut data = "Rust is great!".to_string();

    get_char(data.clone());//深拷贝，所有权不变

    string_uppercase(&mut data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase( data: & mut String) {
    //在原字符串上操作，所以设为动态
    *data = data.to_uppercase();

    println!("{}", data);
}
