use std::fs;
// use std::str;

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
        let (a, b, c) = self.get_side_lengths();
        let mut v = [a, b, c];
        v.sort();
        let [a, b, c] = v;
        return (a, b, c)
    }

    fn calculate_wrapping_paper(&self) -> i32 {
        let (a, b, c) = self.sort_sides();

        return a + 2*a*b + 2*a*c + 2*b*c
    }
}


fn part1() -> Option<Result> {
    let contents = fs::read_to_string("input.txt")
    .expect("Input file is expected");

    let mut total_wrapping_paper = 0;

    for n in contents.lines() {
        println!("Line is {n}");
        let parts = n.split("x");
        let collection = parts.collect::<Vec<&str>>();
        dbg!(&collection);

        let wrapping_box= Box {length: collection[0].parse().unwrap(), width: collection[1].parse().unwrap(), height: collection[2].parse().unwrap()};
        total_wrapping_paper += wrapping_box.calculate_wrapping_paper()

    }

    Some(Result { total_wrapping_paper: total_wrapping_paper, part2: None})
}

// fn part2() -> Result {
//     let contents = fs::read_to_string("input.txt")
//     .expect("Input file is expected");
// 
//     
//     let mut floor = 0;
//     let mut current_character = 0;
// 
//     for n in contents.chars() {
//         current_character += 1;
//         println!("Floor is {floor} next is {n}");
//         if n == '(' {
//             floor += 1
//         } else if n == ')' {
//             floor -= 1
//         } else {
//             panic!("Unexpected character {n}")
//         }
// 
//         if floor < 0 {
//             return Result { floor: floor, current_character: Some(current_character)}
//         }
//     }
// 
//     Result { floor: floor, current_character: None}
// }

fn main() {
    let result1 = part1();
    println!("Final floor is {result1:?}");
    // let result2 = part2();
    // println!("Negative floor is {result2:?}");
}


// Attempted answers
// 464001047 too high
