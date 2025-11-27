
use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Answer {
    answer: usize
}

trait InputGetter {
    fn get_input(&self) -> String;
    
}

struct LocalFileInputGetter {
    path: &'static str,
}

impl InputGetter for LocalFileInputGetter {
    fn get_input(&self) -> String {
        return fs::read_to_string(self.path).expect("Input file is expected");
    }
}

struct HouseLights {
    // lights: Vec<Vec<bool>>,
    lights: [[bool; 1000]; 1000],
}

struct Instruction {
    action: String,
    startx: usize,
    starty: usize,
    endx: usize,
    endy: usize,
}

fn parse_instructions(contents: &String) -> Vec<Instruction> {
    let instructions = vec![];
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let action = &parts[..parts.len()-3];

        // TODO: Tidy these up into one function
        let start: Vec<&str> = parts[parts.len()-3].split(",").collect();
        let startx= start.get(0).expect("Value required here").parse().expect("Not a valid value");
        let starty= start.get(1).expect("Value required here").parse().expect("Not a valid value");

        let end: Vec<&str> = parts[parts.len()-1].split(",").collect();
        let endx= end.get(0).expect("Value required here").parse().expect("Not a valid value");
        let endy= end.get(1).expect("Value required here").parse().expect("Not a valid value");
        println!("{action:?} {startx:?}");
        let instruction = Instruction {action, startx, starty, endx, endy};
    }

    return instructions;
}


fn part1(contents: &String) -> Option<Answer> {
    // Create a 2d array that will store our light positions
    let house_lights = HouseLights {lights: [[true; 1000]; 1000]};

    // Loop through each instruction and set them in house_lights
    let instructions = parse_instructions(contents);


    // loop through all of house_lights and count how many are true

    None

}

// Attempted answers

fn part2(contents: &String) -> Option<Answer> {
    None

}

// Attempted answers

fn main() {

    let contents = LocalFileInputGetter{ path: "input.txt"}.get_input();
    let result1 = part1(&contents);
    println!("Part1 result {result1:?}");

    let result2 = part2(&contents);
    println!("Part2 result {result2:?}");

}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instructions() {
        let instruction = "turn on 0,0 through 999,999\n".to_string();

        let instructions = parse_instructions(&instruction);

        assert_eq!(instructions.len(), 1);

        let out_instruction = instructions.get(0).unwrap();

        assert_eq!(out_instruction.action, "turn on");

        assert_eq!(out_instruction.startx, 0);
        assert_eq!(out_instruction.starty, 0);

        assert_eq!(out_instruction.endx, 999);
        assert_eq!(out_instruction.endy, 999);
    }

    #[test]
    fn test_part1() {
    let contents = LocalFileInputGetter{ path: "input.txt"}.get_input();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 2081}));
    }

    #[ignore]
    #[test]
    fn test_part2() {
    let contents = LocalFileInputGetter{ path: "input.txt"}.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 2341}));
    }
}