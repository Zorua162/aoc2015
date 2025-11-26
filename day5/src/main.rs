use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Answer {
    answer: usize,
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

// trait RuleTrait {
//     fn check_rule(&self, string: String) -> bool;
// }

// #[derive(Debug, PartialEq)]
// struct Rule;
// impl RuleTrait for Rule {
//     fn check_rule(&self, string: String) -> bool {
//
//         return false;
//     }
// }

fn create_vowel_count() -> HashMap<char, u32> {
    let mut vowel_count: HashMap<char, u32> = HashMap::new();
    for char in VOWELS.chars() {
        vowel_count.insert(char, 0);
    }
    return vowel_count
}

#[derive(Debug, PartialEq)]
enum Rule {
    ThreeVowels {},
    DoubleLetter {},
    DisallowedStrings {},
}

static VOWELS: &str = "aeiou";

impl Rule {
    pub fn do_rule(&self, line: String) -> bool {
        match self {
            Rule::ThreeVowels { } => {
                let mut vowel_count: HashMap<char, u32> = HashMap::new();
                for n in line.chars() {
                    if VOWELS.contains(n) {
                        vowel_count.entry(n).and_modify(|counter| *counter += 1).or_insert(1);
                    }
                }
                return vowel_count.values().sum::<u32>() >= 3
            },
            Rule::DoubleLetter { } => false,
            Rule::DisallowedStrings { } => false,
        }
    }

}

struct RuleManager {
    rules: Vec<Rule>,
}

fn part1(contents: &String) -> Option<Answer> {
    let rules = vec![Rule::ThreeVowels {}];
    let ruleManager = RuleManager{rules: rules};
    None
}

// Attempted answers

fn part2(contents: &String) -> Option<Answer> {
    None
}

// Attempted answers

fn main() {
    let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
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
    fn test_three_vowels_rule_true() {
        let input = "one";
        let rule = Rule::ThreeVowels {  };
        assert!(rule.do_rule(input.to_string()));
    }

    #[test]
    fn test_part1() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 2081 }));
    }

    #[test]
    fn test_part2() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 2341 }));
    }
}
