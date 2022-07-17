use std::mem;

enum Operation {
    Add,
    Mul,
}

pub(self) fn simple_infix_math(s: &str) -> u64 {
    let mut res = 0;
    let mut num = 0;
    let mut op = Operation::Add;
    let mut count_nested = 0;
    let mut iter = s.chars().enumerate();

    while let Some((i, ch)) = iter.next() {
        match ch {
            '0'..='9' => {
                num *= 10;
                num += ch.to_digit(10).unwrap() as u64;
            }

            '+' | '*' => {
                match mem::replace(
                    &mut op,
                    match ch {
                        '+' => Operation::Add,
                        '*' => Operation::Mul,
                        _ => unreachable!(),
                    },
                ) {
                    Operation::Add => res += num,
                    Operation::Mul => res *= num,
                };

                num = 0;
            }

            ' ' => (),

            '(' => {
                while let Some((j, c)) = iter.next() {
                    match c {
                        '(' => count_nested += 1,
                        ')' if count_nested != 0 => count_nested -= 1,
                        ')' => {
                            num = simple_infix_math(&s[(i + 1)..j]);
                            break;
                        }
                        _ => (),
                    }
                }
            }

            _ => unreachable!(),
        }
    }

    match op {
        Operation::Add => res += num,
        Operation::Mul => res *= num,
    }

    res
}

pub(self) fn advanced_infix_math(s: &str) -> u64 {
    let mut res = vec![0];
    let mut num = 0;
    let mut op = Operation::Add;
    let mut count_nested = 0;
    let mut iter = s.chars().enumerate();

    while let Some((i, ch)) = iter.next() {
        match ch {
            '0'..='9' => {
                num *= 10;
                num += ch.to_digit(10).unwrap() as u64;
            }

            '+' | '*' => {
                match mem::replace(
                    &mut op,
                    match ch {
                        '+' => Operation::Add,
                        '*' => Operation::Mul,
                        _ => unreachable!(),
                    },
                ) {
                    Operation::Add => *res.last_mut().unwrap() += num,
                    Operation::Mul => res.push(num),
                };

                num = 0;
            }

            ' ' => (),

            '(' => {
                while let Some((j, c)) = iter.next() {
                    match c {
                        '(' => count_nested += 1,
                        ')' if count_nested != 0 => count_nested -= 1,
                        ')' => {
                            num = advanced_infix_math(&s[(i + 1)..j]);
                            break;
                        }
                        _ => (),
                    }
                }
            }

            _ => unreachable!(),
        }
    }

    match op {
        Operation::Add => *res.last_mut().unwrap() += num,
        Operation::Mul => res.push(num),
    }

    res.into_iter().product()
}

pub struct OperationOrder {
    input: String,
}

impl crate::AdventOfCode for OperationOrder {
    const TITLE: &'static str = "Operation Order";
    const DAY: u8 = 18;

    fn new(input: &str) -> Option<Self> {
        let input = input.to_owned();
        Some(Self { input })
    }

    fn part1(&self) -> u64 {
        self.input.lines().map(simple_infix_math).sum()
    }

    fn part2(&self) -> u64 {
        self.input.lines().map(advanced_infix_math).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_math_1() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        let res = simple_infix_math(input);
        assert_eq!(res, 71);
    }

    #[test]
    fn simple_math_2() {
        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        let res = simple_infix_math(input);
        assert_eq!(res, 51);
    }

    #[test]
    fn simple_math_3() {
        let input = "2 * 3 + (4 * 5)";
        let res = simple_infix_math(input);
        assert_eq!(res, 26);
    }

    #[test]
    fn simple_math_4() {
        let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let res = simple_infix_math(input);
        assert_eq!(res, 437);
    }

    #[test]
    fn simple_math_5() {
        let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let res = simple_infix_math(input);
        assert_eq!(res, 12240);
    }

    #[test]
    fn simple_math_6() {
        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let res = simple_infix_math(input);
        assert_eq!(res, 13632);
    }

    #[test]
    fn advanced_math_1() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        let res = advanced_infix_math(input);
        assert_eq!(res, 231);
    }

    #[test]
    fn advanced_math_2() {
        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        let res = advanced_infix_math(input);
        assert_eq!(res, 51);
    }

    #[test]
    fn advanced_math_3() {
        let input = "2 * 3 + (4 * 5)";
        let res = advanced_infix_math(input);
        assert_eq!(res, 46);
    }

    #[test]
    fn advanced_math_4() {
        let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let res = advanced_infix_math(input);
        assert_eq!(res, 1445);
    }

    #[test]
    fn advanced_math_5() {
        let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let res = advanced_infix_math(input);
        assert_eq!(res, 669060);
    }

    #[test]
    fn advanced_math_6() {
        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let res = advanced_infix_math(input);
        assert_eq!(res, 23340);
    }
}
