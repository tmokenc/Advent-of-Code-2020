struct CorupttedPassword {
    password: String,
    char_hint: char,
    min: usize,
    max: usize,
}

pub struct PasswordPhilosophy {
    list: Vec<CorupttedPassword>,
}

impl crate::AdventOfCode for PasswordPhilosophy {
    const TITLE: &'static str = "Password Philosophy";
    const DAY: u8 = 2;

    fn new(input: &str) -> Option<Self> {
        let mut list = Vec::new();

        for line in input.lines() {
            let mut iter = line.split_whitespace();
            let mut length = iter.next()?.split('-');

            let min = length.next()?.parse::<usize>().ok()?;
            let max = length.next()?.parse::<usize>().ok()?;
            let char_hint = iter.next()?.chars().next()?;
            let password = iter.next()?.to_owned();

            list.push(CorupttedPassword {
                password,
                char_hint,
                min,
                max,
            });
        }

        Some(Self { list })
    }

    fn part1(&self) -> u64 {
        self.list
            .iter()
            .filter(|data| {
                let count = data
                    .password
                    .chars()
                    .filter(|&ch| ch == data.char_hint)
                    .count();

                count >= data.min && count <= data.max
            })
            .count() as u64
    }

    fn part2(&self) -> u64 {
        self.list
            .iter()
            .filter(|data| {
                let mut iter = data.password.chars();
                let a = iter.nth(data.min - 1).unwrap();
                let b = iter.nth(data.max - data.min - 1).unwrap();
                (a != b) && (a == data.char_hint || b == data.char_hint)
            })
            .count() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn part1() {
        let input = include_str!("../example_input/day02.txt");
        let res = PasswordPhilosophy::new_unwrap(input);
        assert_eq!(res.part1(), 2);
    }

    #[test]
    fn part2() {
        let input = include_str!("../example_input/day02.txt");
        let res = PasswordPhilosophy::new_unwrap(input);
        assert_eq!(res.part2(), 1);
    }
}
