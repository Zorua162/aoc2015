use std::fs;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Result {
    floor: i32,
    current_character: Option<i32>
}

fn part1() -> Result {
    let contents = fs::read_to_string("input.txt")
    .expect("Input file is expected");

    
    let mut floor = 0;

    for n in contents.chars() {
        println!("Floor is {floor} next is {n}");
        if n == '(' {
            floor += 1
        } else if n == ')' {
            floor -= 1
        } else {
            panic!("Unexpected character {n}")
        }

    }

    Result { floor: floor, current_character: None }
}

fn part2() -> Result {
    let contents = fs::read_to_string("input.txt")
    .expect("Input file is expected");

    
    let mut floor = 0;
    let mut current_character = 0;

    for n in contents.chars() {
        current_character += 1;
        println!("Floor is {floor} next is {n}");
        if n == '(' {
            floor += 1
        } else if n == ')' {
            floor -= 1
        } else {
            panic!("Unexpected character {n}")
        }

        if floor < 0 {
            return Result { floor: floor, current_character: Some(current_character)}
        }
    }

    Result { floor: floor, current_character: None}
}

fn main() {
    let result1 = part1();
    println!("Final floor is {result1:?}");
    let result2 = part2();
    println!("Negative floor is {result2:?}");
}