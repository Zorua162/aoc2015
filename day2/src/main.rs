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
        println!("Sorted sides are {a}, {b}, {c}");

        return a + 2*a + 2*b + 2*c
    }

    fn sort_lengths(&self) -> (i32, i32, i32) {
        let mut v = [self.length, self.width, self.height];
        v.sort();
        let [a, b, c] = v;
        return (a, b, c)

    }

    fn calculate_ribbon(&self) -> i32 {
        let (a, b, c) = self.sort_lengths();
        return 2*a + 2*b + a*b*c
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

// Part1 Attempted answers
// 464001047 too high


fn part2() -> Option<Result> {
    let contents = fs::read_to_string("input.txt")
    .expect("Input file is expected");

    let mut total_ribbon = 0;

    for n in contents.lines() {
        println!("Line is {n}");
        let parts = n.split("x");
        let collection = parts.collect::<Vec<&str>>();
        dbg!(&collection);

        let wrapping_box= Box {length: collection[0].parse().unwrap(), width: collection[1].parse().unwrap(), height: collection[2].parse().unwrap()};
        total_ribbon += wrapping_box.calculate_ribbon()

    }

    Some(Result { total_wrapping_paper: 0, part2: Some(total_ribbon)})
}


// Part2 attempted answers
// 1606483 - too low
// 3842356 - correct!


fn main() {
    let result1 = part1();
    println!("{result1:?}");
    let result2 = part2();
    println!("{result2:?}");
}
