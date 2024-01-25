fn main() {
    foo();
}

fn foo() -> Vec<char> {
    let mut data = vec!['a', 'b', 'c']; // --+ 'scope
    let slice = &mut data[..]; //<-------+ 'lifetime
    capitalize(slice); //  |
    data.push('d');
    data.push('e');
    data.push('f');
    data
}

fn capitalize(slice: &mut [char]) {}
