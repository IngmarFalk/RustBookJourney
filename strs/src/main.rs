fn main() {
    println!("{}", strs_push_str());

    println!("{}", strs());

    println!("{}", strs_format())
}

fn strs_push_str() -> String {
    let mut s: String = String::from("Hello");
    s.push_str(", world.");
    s
}

fn strs() -> String {
    "Hello".to_string() + &", world!"
}

fn strs_format() -> String {
    format!("{}{}", "Hello", ", world?")
}
