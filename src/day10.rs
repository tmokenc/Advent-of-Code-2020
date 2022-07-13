pub struct AdapterArray {
    adapters: Vec<u64>,
}

impl crate::AdventOfCode for AdapterArray {
    const TITLE: &'static str = "Adapter Array";
    const DAY: u8 = 10;

    fn new(input: &str) -> Option<Self> {
        let mut list = crate::lines_to_vec::<u64>(input)?;
        list.push(0);
        list.sort_unstable();
        list.push(list.last().copied().unwrap_or(0) + 3);

        Some(Self { adapters: list })
    }

    fn part1(&self) -> u64 {
        let mut by1 = 0;
        let mut by3 = 0;

        for i in 1..self.adapters.len() {
            match self.adapters[i] - self.adapters[i - 1] {
                1 => by1 += 1,
                3 => by3 += 1,
                _ => (),
            };
        }
        by1 * by3
    }

    fn part2(&self) -> u64 {
        let mut ways = std::collections::HashMap::<u64, u64>::new();
        ways.insert(0, 1);

        for v in self.adapters[1..].iter().copied() {
            let mut num = ways.get(&(v - 1)).copied().unwrap_or(0);

            if v > 1 {
                num += ways.get(&(v - 2)).unwrap_or(&0);
            }

            if v > 2 {
                num += ways.get(&(v - 3)).unwrap_or(&0);
            }

            *ways.entry(v).or_insert(0) += num;
        }

        *ways.get(self.adapters.last().unwrap()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn part1() {
        let input = include_str!("../example_input/day10.txt");
        let res = AdapterArray::new_unwrap(input);
        assert_eq!(res.part1(), 220);
    }

    #[test]
    fn part2() {
        let input = include_str!("../example_input/day10.txt");
        let res = AdapterArray::new_unwrap(input);
        assert_eq!(res.part2(), 19208);
    }
}
