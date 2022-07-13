use std::collections::HashSet;

type Answers = Vec<String>;

fn count_answers(answers: &[String]) -> u64 {
    answers
        .iter()
        .flat_map(|v| v.chars())
        .collect::<HashSet<char>>()
        .len() as u64
}

fn count_same_answers(answers: &[String]) -> u64 {
    if answers.is_empty() {
        return 0;
    }

    if answers.len() == 1 {
        return answers[0].len() as u64;
    }

    answers[0]
        .chars()
        .filter(|&ch| {
            for other in &answers[1..] {
                if other.chars().position(|v| v == ch).is_none() {
                    return false;
                }
            }

            true
        })
        .count() as u64
}

pub struct CustomCustoms {
    groups: Vec<Answers>,
}

impl crate::AdventOfCode for CustomCustoms {
    const TITLE: &'static str = "Custom Customs";
    const DAY: u8 = 6;

    fn new(input: &str) -> Option<Self> {
        let mut groups = Vec::new();

        for data in input.split("\n\n") {
            let answers = data.lines().map(String::from).collect();
            groups.push(answers);
        }

        Some(Self { groups })
    }

    fn part1(&self) -> u64 {
        self.groups.iter().map(|v| count_answers(v)).sum()
    }

    fn part2(&self) -> u64 {
        self.groups.iter().map(|v| count_same_answers(v)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn part1() {
        let input = include_str!("../example_input/day06.txt");
        let res = CustomCustoms::new_unwrap(input);
        assert_eq!(res.part1(), 11);
    }

    #[test]
    fn part2() {
        let input = include_str!("../example_input/day06.txt");
        let res = CustomCustoms::new_unwrap(input);
        assert_eq!(res.part2(), 6);
    }
}
