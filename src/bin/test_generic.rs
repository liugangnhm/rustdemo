use std::{fmt::Debug, vec};

struct Cmd1;

struct Cmd2;

struct Cmd3;

struct Result1;

struct Result2;

trait Parse<RESULT> {
    fn parse(data: &Vec<u8>) -> RESULT;
}

impl Parse<i32> for Cmd1 {
    fn parse(data: &Vec<u8>) -> i32 {
        data.iter().map(|&x| x as i32).sum()
    }
}
#[derive(Debug, Clone, Copy)]
struct S1 {
    i: i32,
    j: i32,
}

fn main() {
    let data = vec![1, 2, 3, 4];
    let sum: i32 = data.iter().map(|&x| x as i32).sum();
    let sum2: i32 = data.iter().sum();

    let data2 = vec![S1 { i: 1, j: 2 }, S1 { i: 3, j: 4 }];

    let sum: i32 = data2.iter().map(|x| x.i + x.j).sum();

    // let sum3: i32 = data2.iter().map(|&x| x.i + x.j).sum();

    println!("{} {}", sum, sum2);
}
