
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

#[derive(Clone, Debug)]
struct HouseLights {
    lights: Vec<Vec<bool>>,
}

impl Default for HouseLights {
    fn default() -> Self {
        // let initial_lights = [[true; 1000]; 1000];
        // let mut lights = vec![vec![]];
        // let lights = initial_lights.to_vec();
        let lights = vec![vec![false; 1000]; 1000];
        Self { lights }
    }
}

#[derive(Clone)]
struct Instruction {
    action: String,
    startx: usize,
    starty: usize,
    endx: usize,
    endy: usize,
}

fn parse_instructions(contents: &String) -> Vec<Instruction> {
    let mut instructions = vec![];
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        println!("{parts:?}");
        let action = parts[..parts.len()-3].iter().map(|s|-> String {s.to_string()}).collect::<Vec<String>>().join(" ");

        // TODO: Tidy these up into one function
        let start: Vec<&str> = parts[parts.len()-3].split(",").collect();
        let startx= start.get(0).expect("Value required here").parse().expect("Not a valid value");
        let starty= start.get(1).expect("Value required here").parse().expect("Not a valid value");

        let end: Vec<&str> = parts[parts.len()-1].split(",").collect();
        let endx= end.get(0).expect("Value required here").parse().expect("Not a valid value");
        let endy= end.get(1).expect("Value required here").parse().expect("Not a valid value");
        let instruction = Instruction {action, startx, starty, endx, endy};
        instructions.push(instruction)
    }

    return instructions;
}

fn apply_instruction(house_lights: HouseLights, instruction: Instruction) -> HouseLights {

    let mut house_lights: HouseLights = house_lights.clone();

   for i in instruction.startx..instruction.endx {
    for j in instruction.starty..instruction.endy {
        match instruction.action.as_str() {
            "turn off" => house_lights.lights[i][j] = false,
            "turn on" => house_lights.lights[i][j] = true,
            "toggle" => house_lights.lights[i][j] = !house_lights.lights[i][j],
            &_ => panic!("Unknown instruction given")
        }
        
    }
   }
   house_lights
}

fn apply_instructions(house_lights: HouseLights, instructions: Vec<Instruction>) -> HouseLights {

    let mut house_lights = house_lights;

    for instruction in instructions {
        house_lights = apply_instruction(house_lights.clone(), instruction);
    }
    house_lights
}

fn count_lights(house_lights: HouseLights) -> usize {
    let mut count: usize = 0;

    let lights = house_lights.lights;

    let first_row = lights.get(0).unwrap();

    for i in 0..lights.len()-1 {
        for j in 0..first_row.len()-1 {
            if lights[i][j] {
                count += 1;
            }
        }
    }

    count
}


fn part1(contents: &String) -> Option<Answer> {
    // Create a 2d array that will store our light positions
    let mut house_lights = HouseLights { ..Default::default() };

    // Loop through each instruction and set them in house_lights
    let instructions = parse_instructions(contents);

    // loop through all of house_lights and count how many are true
    house_lights = apply_instructions(house_lights, instructions);

    let answer = count_lights(house_lights);

    Some(Answer{ answer })

}

// Attempted answers
// 398967 too low

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
    fn test_apply_instruction() {
        let instruction = "turn on 0,0 through 999,999\n".to_string();

        let instructions = parse_instructions(&instruction);

        let out_instruction = instructions.get(0).unwrap();

        let mut house_lights = HouseLights { ..Default::default() };

        // loop through all of house_lights and count how many are true
        house_lights = apply_instruction(house_lights, out_instruction.clone());

        assert_eq!(true, *house_lights.lights.get(0).unwrap().get(0).unwrap());
    }

    #[test]
    fn test_apply_instruction_line() {
        let instruction = "turn on 0,0 through 999,0\n".to_string();

        let instructions = parse_instructions(&instruction);

        let out_instruction = instructions.get(0).unwrap();

        let mut house_lights = HouseLights { ..Default::default() };

        // loop through all of house_lights and count how many are true
        house_lights = apply_instruction(house_lights, out_instruction.clone());

        let count = count_lights(house_lights);

        assert_eq!(count, 1000);

    }

    #[test]
    fn test_apply_instructions() {
        let instruction = "turn on 0,0 through 999,999\n\
                                   turn on 0,0 through 499,499".to_string();

        let instructions = parse_instructions(&instruction);

        let out_instruction = instructions.get(0).unwrap();

        let mut house_lights = HouseLights { ..Default::default() };

        // loop through all of house_lights and count how many are true
        house_lights = apply_instruction(house_lights, out_instruction.clone());

        let count = count_lights(house_lights);

        assert_eq!(count, 500000)
    }

    #[ignore]
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