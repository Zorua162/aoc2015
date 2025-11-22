use std::fs;
use md5::{Md5, Digest};

#[derive(Debug, PartialEq)]
struct Answer {
    answer: u32
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


fn hash_value(value: String) -> String {
    let mut hasher = Md5::new();
    hasher.update(value.as_bytes());
    let result = hasher.finalize();
    return format!("{result:x}")
}

fn find_hash_with_starting(contents: &String, starting: &str, start_search: u32, max_search: u32) -> Option<Answer> {
    for i in start_search..max_search {

        let value = format!("{}{}", contents, i);

        // println!("value is {value}");

        let result = hash_value(value);

        let initial_five = &result[..5];

        // println!("initial five are {initial_five}");

        if i % 1000000 == 0 {
            println!{"Current iteration {i}"}
        }
        
        if initial_five == starting {
            println!("Result {result}");
            return Some(Answer{ answer: i } )
            
        }
    }

    // Answer wasn't found
    return None;

}


fn part1(contents: &String) -> Option<Answer> {
    return find_hash_with_starting(contents, "00000", 0, 1000000 );
}


fn part2(contents: &String) -> Option<Answer> {
    return find_hash_with_starting(contents, "000000", 735000000, 1000000000);
}

fn main() {

    let contents = LocalFileInputGetter{ path: "input.txt"}.get_input();

    // Part one is brute force the md5 by going up one number at a time
    // let result1 = part1(&contents);
    // println!("Part1 result {result1:?}");

    // Part two seems to be brute force, but with even more pizzaz
    let result2 = part2(&contents);
    println!("Part2 result {result2:?}");

}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
    let contents = LocalFileInputGetter{ path: "input.txt"}.get_input();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 117946}));
    }

    #[test]
    fn test_part2() {
    let contents = LocalFileInputGetter{ path: "input.txt"}.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 2341}));
    }
}