use crate::AdventOfCode;

pub struct ReportPair {
    list: Vec<u16>,
}

impl AdventOfCode for ReportPair {
    const TITLE: &'static str = "Report Pair";
    const DAY: u8 = 1;

    fn new(input: &str) -> Option<Self> {
        Some(Self {
            list: crate::lines_to_vec::<u16>(input)?,
        })
    }

    fn part1(&self) -> u64 {
        for (idx, &x) in self.list.iter().enumerate() {
            for &v in &self.list[idx + 1..] {
                if x + v == 2020 {
                    return x as u64 * v as u64;
                }
            }
        }

        0
    }

    fn part2(&self) -> u64 {
        for (idx, &x) in self.list.iter().enumerate() {
            for (idx2, &v) in self.list[idx + 1..].iter().enumerate() {
                for &y in &self.list[idx2 + 1..] {
                    if x + v + y == 2020 {
                        return x as u64 * v as u64 * y as u64;
                    }
                }
            }
        }

        0
    }
}
