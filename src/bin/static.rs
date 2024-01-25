use std::str::Chars;

// 错误，为什么？
fn lifetime1() -> &'static str {
    "Tyr"
}

// 错误，为什么？
fn lifetime2(name: &String) -> &str {
    &name[1..]
}

// 正确，为什么？
fn lifetime3(name: &str) -> Chars {
    name.chars()
}

fn main() {}
