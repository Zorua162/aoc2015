
use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Answer {
    answer: usize
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn move_location(self: &mut Location, direction: &char) {
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


fn part1(contents: &String) -> Option<Answer> {

    let mut houses: HashMap<Location, i32> = HashMap::new();

    let mut current_location = Location{ x: 0, y: 0 };

    houses.insert(current_location.clone(), 1);


    for n in contents.chars() {
        current_location.move_location(&n);

        houses.entry(current_location.clone()).and_modify(|counter| *counter += 1).or_insert(1);

    }

    println!("All houses are {houses:?}");
    let num_houses = houses.len();

    return Some(Answer{ answer: num_houses } )
}

// Attempted answers
// 2080 too low (one off error!)

fn part2(contents: &String) -> Option<Answer> {

    let mut houses: HashMap<Location, i32> = HashMap::new();

    let mut santa_location = Location{ x: 0, y: 0 };
    let mut robo_santa_location = Location{ x: 0, y: 0 };

    houses.insert(santa_location.clone(), 1);


    for (i, n) in contents.chars().enumerate() {
        if i % 2 == 0 {
            santa_location.move_location(&n);
            houses.entry(santa_location.clone()).and_modify(|counter| *counter += 1).or_insert(1);
        } else {
            robo_santa_location.move_location(&n);
            houses.entry(robo_santa_location.clone()).and_modify(|counter| *counter += 1).or_insert(1);

        }

    }

    // println!("All houses are {houses:?}");
    let num_houses = houses.len();

    return Some(Answer{ answer: num_houses } )
}

// Attempted answers
// 1150 too low

fn main() {
    // Planning
    // Track x and y
    // Add to list x,y (if it doesn't already exist)
    // Length of list is part 1

    // Part 2 is probably going to involve house most visited
    // Extend list to map, if it exists then add one
    let contents = fs::read_to_string("input.txt")
    .expect("Input file is expected");

    let result1 = part1(&contents);
    println!("Part1 result {result1:?}");

    // Plan was wrong, instead there are two sleighs for some reason?!?!

    let result2 = part2(&contents);
    println!("Part2 result {result2:?}");

}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
    let contents = fs::read_to_string("input.txt")
    .expect("Input file is expected");
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 2081}));
    }

    #[test]
    fn test_part2() {
    let contents = fs::read_to_string("input.txt")
    .expect("Input file is expected");
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 2341}));
    }
}