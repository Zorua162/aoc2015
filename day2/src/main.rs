use std::fs;

#[derive(Debug)]
struct Result {
    total_wrapping_paper: i32,
    part2: Option<i32>
}

struct Box {
    length: i32,
    width: i32,
    height: i32
}

impl Box {

    fn get_side_lengths(&self) -> (i32, i32, i32) {
        return (self.length*self.width, self.width*self.height, self.height*self.length)
    }

    fn sort_sides(&self) -> (i32, i32, i32) {
        return self.get_side_lengths()
    }
}


fn part1() -> Option<Result> {
    let contents = fs::read_to_string("input.txt")
    .expect("Input file is expected");

    let mut total_wrapping_paper = 0;

    for n in contents.lines() {
        println!("Line is {n}");

    }

    None
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
    // let result2 = part2();
    // println!("Negative floor is {result2:?}");
}