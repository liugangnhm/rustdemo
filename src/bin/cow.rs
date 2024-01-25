use std::borrow::Cow;
fn remove_spaces<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains(' ') {
        let mut buf = String::with_capacity(input.len());
        for c in input.chars() {
            if c != ' ' {
                buf.push(c);
            }
        }
        return Cow::Owned(buf);
    }
    return Cow::Borrowed(input);
}

fn main() {
    let s1 = "no_spaces_in_string";
    let result1 = remove_spaces(s1);
    let s2 = "spaces in string";
    let result2 = remove_spaces(s2);
    println!("{}\n{}", result1, result2);
}
