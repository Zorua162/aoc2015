
use std::fs;
use std::collections::HashMap;

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

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn move_location(mut self: &mut Location, direction: &char) {
        print!("{direction}");
        match direction {
            '^' => &self.up(),
            'v' => &self.down(),
            '<' => &self.left(),
            '>' => &self.right(),
            _ => panic!("direction {direction} was not recognized as a valid direction"),
        };
    }

    fn up(&mut self) {
        self.y += 1;

    }

    fn down(&mut self) {
        self.y -= 1;

    }

    fn left(&mut self) {
        self.x += 1;
    }

    fn right(&mut self) {
        self.x -= 1;
    }
}


fn part1() -> Option<Result> {
    let contents = fs::read_to_string("input.txt")
    .expect("Input file is expected");

    let mut houses: HashMap<Location, i32> = HashMap::new();

    let mut current_location = Location{ x: 0, y: 0 };



    for n in contents.chars() {

        let mut default_visited_value = 0;
        current_location.move_location(&n);

        // let visited: &mut i32 = houses.get_mut(&current_location).unwrap_or(&mut default_visited_value);

        // *visited += 1;

        // houses.insert(current_location.clone(), *visited);

        houses.entry(current_location.clone()).and_modify(|counter| *counter += 1).or_insert(1);

        // let mut visited: &mut i32 = houses.get_mut(&current_location).unwrap_or(&mut default_visited_value);
        // *visited += 1;
        // houses.insert(current_location, *visited);
    }


    println!("All houses are {houses:?}");
    let num_houses = houses.len();
    println!("There are {num_houses} houses");

    None
}

// Attempted answers
// 2080 too low

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
    // Track x and y
    // Add to list x,y (if it doesn't already exist)
    // Length of list is part 1

    // Part 2 is probably going to involve house most visited
    // Extend list to map, if it exists then add one
}


// Attempted answers
