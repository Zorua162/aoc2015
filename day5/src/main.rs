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


#[derive(Debug, PartialEq)]
enum Rule {
    ThreeVowels {},
    DoubleLetter {},
    DisallowedStrings {},
    PairAnyTwo {},
    SkipDouble {},
}

static VOWELS: &str = "aeiou";
static DISALLOWED: [&str; 4] = ["ab", "cd", "pq", "xy"];


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
            Rule::DoubleLetter { } => {
                for (char1, char2) in line.chars().zip(line.chars().skip(1)) {
                        if char1 == char2 {
                            return true;
                        }
                }
                false
             },
            Rule::DisallowedStrings { } => {
                for find in DISALLOWED.iter() {
                    if line.contains(find) {
                        return false;
                    }
                }
                true
            },
            Rule::PairAnyTwo { } => {
                // Bad method, as it doesn't account for strings like aaa not counting
                // Hashmap which gets added to for a letter when a double is found, equals 2
                // let mut double_count: HashMap<char, u32> = HashMap::new();
                // for (char1, char2) in line.chars().zip(line.chars().skip(1)) {
                //         if char1 == char2 {
                //             double_count.entry(char1).and_modify(|counter| *counter += 1).or_insert(1);
                //         }
                // }

                // for count in double_count.values() {
                //     if count == &2 {
                //         return true;
                //     }
                // }
                // let vec_line: Vec<char> = line.chars().collect();
                // let mut double_count: HashMap<String, u32> = HashMap::new();
                // let mut blocked_loc: Vec<usize> = vec![];
                // for (i, n) in vec_line.iter().enumerate() {
                //     if !(i+1>=vec_line.len()) && !(blocked_loc.contains(&i)) && *n == vec_line[i+1] {
                //         let key = format!("{}{}", *n, vec_line[i+1]);
                //         double_count.entry(key).and_modify(|counter| *counter += 1).or_insert(1);
                //         blocked_loc.push(i+1);
                //     }
                // }

                // Get every pair of letters
                // Count how many times they appear - remove them as they are counted
                // if a pair appears twice then this rule is followed
                
                // With the exception that a triple gets one removed from it's pair count
                let mut pair_count: HashMap<String, u32> = HashMap::new();

                for (char1, char2) in line.chars().zip(line.chars().skip(1)) {
                    let pair = format!("{}{}", char1, char2);
                    pair_count.entry(pair).and_modify(|counter| *counter += 1).or_insert(1);

                }

                // Remove one from triple characters
                for ((char1, char2), char3) in line.chars().zip(line.chars().skip(1)).zip(line.chars().skip(2)) {
                    if char1 == char2 && char2 == char3 {
                        let pair = format!("{}{}", char1, char2);
                        pair_count.entry(pair).and_modify(|counter| *counter -= 1).or_insert(1);
                    }
                }

                // println!("pair_counting: {pair_count:?}");
                for count in pair_count.values() {
                    if count >= &2 {
                        return true;
                    }
                }
                false
             },
            Rule::SkipDouble { } => {
                for (char1, char2) in line.chars().zip(line.chars().skip(2)) {
                        if char1 == char2 {
                            return true;
                        }
                }
                false
             },
        }
    }

}

struct RuleManager {
    rules: Vec<Rule>,
}

impl RuleManager {
    pub fn check_rules(&self, line: &str) -> bool {
        for rule in self.rules.iter() {
            if !(rule.do_rule(line.to_string())) {
                return false
            }
        }
        true
    }
}

fn part1(contents: &String) -> Option<Answer> {
    let rules = vec![Rule::ThreeVowels {}, Rule::DoubleLetter {  }, Rule::DisallowedStrings {  }];
    let rule_manager = RuleManager{rules: rules};

    let mut remaining: Vec<&str> = vec![];

    for line in contents.lines() {
        if rule_manager.check_rules(line) {
            remaining.push(line);
        }
    }
    Some(Answer{answer: remaining.len()})
}

// Attempted answers
// 248 too low

fn part2(contents: &String) -> Option<Answer> {
    let rules = vec![Rule::SkipDouble {  }, Rule::PairAnyTwo {  }];
    let rule_manager = RuleManager{rules: rules};

    let mut remaining: Vec<&str> = vec![];

    for line in contents.lines() {
        if rule_manager.check_rules(line) {
            remaining.push(line);
        }
    }
    Some(Answer{answer: remaining.len()})
}

// Attempted answers
// 25 not right
// 52

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
        let input = "aei";
        let rule = Rule::ThreeVowels {  };
        let out = rule.do_rule(input.to_string());
        assert_eq!(out, true);
    }

    #[test]
    fn test_three_vowels_rule_false() {
        let input = "bbbcde";
        let rule = Rule::ThreeVowels {  };
        let out = rule.do_rule(input.to_string());
        assert_eq!(out, false);
    }

    #[test]
    fn test_double_letters_rule_true() {
        let input = "abbcd";
        let rule = Rule::DoubleLetter {  };
        let out = rule.do_rule(input.to_string());
        assert_eq!(out, true);
    }

    #[test]
    fn test_double_letters_rule_false() {
        let input = "abcde";
        let rule = Rule::DoubleLetter {  };
        let out = rule.do_rule(input.to_string());
        assert_eq!(out, false);
    }

    #[test]
    fn test_disallowed_strings_rule_true() {
        let input = "aaaaa";
        let rule = Rule::DisallowedStrings {  };
        let out = rule.do_rule(input.to_string());
        assert_eq!(out, true);
    }

    #[test]
    fn test_disallowed_strings_rule_false() {
        // ab is disallowed
        let input = "abcde";
        let rule = Rule::DisallowedStrings {  };
        let out = rule.do_rule(input.to_string());
        assert_eq!(out, false);
    }
    #[test]
    fn test_end_to_end_nice_1() {
        // Should be a "nice" string (included)
        let input = "ugknbfddgicrmopn";
        let rules = vec![Rule::ThreeVowels {}, Rule::DoubleLetter {  }, Rule::DisallowedStrings {  }];
        let rule_manager = RuleManager{rules: rules};
        let result = rule_manager.check_rules(input);
        assert_eq!(result, true);
    }

    #[test]
    fn test_end_to_end_nice_2() {
        // Should be a "nice" string (included)
        let input = "aaa";
        let rules = vec![Rule::ThreeVowels {}, Rule::DoubleLetter {  }, Rule::DisallowedStrings {  }];
        let rule_manager = RuleManager{rules: rules};
        let result = rule_manager.check_rules(input);
        assert_eq!(result, true);
    }

    #[test]
    fn test_end_to_end_naught_1() {
        // Should be a "nice" string (included)
        let input = "jchzalrnumimnmhp";
        let rules = vec![Rule::ThreeVowels {}, Rule::DoubleLetter {  }, Rule::DisallowedStrings {  }];
        let rule_manager = RuleManager{rules: rules};
        let result = rule_manager.check_rules(input);
        assert_eq!(result, false);
    }

    #[test]
    fn test_end_to_end_naught_2() {
        // Should be a "nice" string (included)
        let input = "haegwjzuvuyypxyu";
        let rules = vec![Rule::ThreeVowels {}, Rule::DoubleLetter {  }, Rule::DisallowedStrings {  }];
        let rule_manager = RuleManager{rules: rules};
        let result = rule_manager.check_rules(input);
        assert_eq!(result, false);
    }

    #[test]
    fn test_end_to_end_naught_3() {
        // Should be a "nice" string (included)
        let input = "dvszwmarrgswjxmb";
        let rules = vec![Rule::ThreeVowels {}, Rule::DoubleLetter {  }, Rule::DisallowedStrings {  }];
        let rule_manager = RuleManager{rules: rules};
        let result = rule_manager.check_rules(input);
        assert_eq!(result, false);
    }

    #[test]
    fn test_part1() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 255 }));
    }

    #[test]
    fn test_pair_any_two_letters_rule_false() {
        let input = "abcde";
        let rule = Rule::PairAnyTwo {  };
        let out = rule.do_rule(input.to_string());
        assert_eq!(out, false);
    }

    #[test]
    fn test_pair_any_two_letters_rule_true() {
        let input = "aabcdefgaa";
        let rule = Rule::PairAnyTwo {  };
        let out = rule.do_rule(input.to_string());
        assert_eq!(out, true);
    }

    #[test]
    fn test_pair_any_two_letters_rule_overlaps_are_false() {
        let input = "aaa";
        let rule = Rule::PairAnyTwo {  };
        let out = rule.do_rule(input.to_string());
        assert_eq!(out, false);
    }

    #[test]
    fn test_skip_double_letters_rule_false() {
        let input = "abcde";
        let rule = Rule::SkipDouble {  };
        let out = rule.do_rule(input.to_string());
        assert_eq!(out, false);
    }

    #[test]
    fn test_skip_double_letters_rule_true() {
        let input = "abcdefeghi";
        let rule = Rule::SkipDouble {  };
        let out = rule.do_rule(input.to_string());
        assert_eq!(out, true);
    }

    #[test]
    fn test_part2_nice_pair_any_two() {
        // Should be a "nice" string (included)
        let input = "qjhvhtzxzqqjkmpb";
        let rule = Rule::PairAnyTwo {  };
        let out = rule.do_rule(input.to_string());
        assert_eq!(out, true);
    }

    #[test]
    fn test_part2_nice_pair_any_two_obscure() {
        // Should be a "nice" string (included)
        let input = "aaabcaa";
        let rule = Rule::PairAnyTwo {  };
        let out = rule.do_rule(input.to_string());
        assert_eq!(out, true);
    }

    #[test]
    fn test_part2_nice_pair_any_two_self_created() {
        // Should be a "nice" string (included)
        let input = "tsatsbts";
        let rule = Rule::PairAnyTwo {  };
        let out = rule.do_rule(input.to_string());
        assert_eq!(out, false);
    }

    #[test]
    fn test_part2_nice_skip_double() {
        // Should be a "nice" string (included)
        let input = "qjhvhtzxzqqjkmpb";
        let rule = Rule::SkipDouble {  };
        let out = rule.do_rule(input.to_string());
        assert_eq!(out, true);
    }

    #[test]
    fn test_part2_nice_skip_double_2() {
        // Should be a "nice" string (included)
        let input = "aaa";
        let rule = Rule::SkipDouble {  };
        let out = rule.do_rule(input.to_string());
        assert_eq!(out, true);
    }

    #[test]
    fn test_part2_end_to_end_nice_1() {
        // Should be a "nice" string (included)
        let input = "qjhvhtzxzqqjkmpb";
        let rules = vec![Rule::SkipDouble {  }, Rule::PairAnyTwo {  }];
        let rule_manager = RuleManager{rules: rules};
        let result = rule_manager.check_rules(input);
        assert_eq!(result, true);
    }

    #[test]
    fn test_part2_end_to_end_nice_2() {
        // Should be a "nice" string (included)
        let input = "xxyxx";
        let rules = vec![Rule::SkipDouble {  }, Rule::PairAnyTwo {  }];
        let rule_manager = RuleManager{rules: rules};
        let result = rule_manager.check_rules(input);
        assert_eq!(result, true);
    }

    #[test]
    fn test_part2_end_to_end_naughty_1() {
        // Should be a "nice" string (included)
        let input = "uurcxstgmygtbstg";
        let rules = vec![Rule::SkipDouble {  }, Rule::PairAnyTwo {  }];
        let rule_manager = RuleManager{rules: rules};
        let result = rule_manager.check_rules(input);
        assert_eq!(result, false);
    }

    #[test]
    fn test_part2_end_to_end_naughty_2() {
        // Should be a "nice" string (included)
        let input = "ieodomkazucvgmuy";
        let rules = vec![Rule::SkipDouble {  }, Rule::PairAnyTwo {  }];
        let rule_manager = RuleManager{rules: rules};
        let result = rule_manager.check_rules(input);
        assert_eq!(result, false);
    }

    #[test]
    fn test_part2_end_to_end_nice_self_1() {
        // Should be a "nice" string (included)
        let input = "tstwsswswrxlzrqs";
        let rules = vec![Rule::SkipDouble {  }, Rule::PairAnyTwo {  }];
        let rule_manager = RuleManager{rules: rules};
        let result = rule_manager.check_rules(input);
        assert_eq!(result, true);
    }

    #[ignore]
    #[test]
    fn test_part2() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 2341 }));
    }
}
