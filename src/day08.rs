#[derive(PartialEq, Clone, Copy)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

pub struct HandheldHalting {
    instructions: Vec<(Operation, i32)>,
}

impl crate::AdventOfCode for HandheldHalting {
    const TITLE: &'static str = "Handheld Halting";
    const DAY: u8 = 8;

    fn new(input: &str) -> Option<Self> {
        let mut instructions = Vec::new();

        for line in input.lines() {
            let mut iter = line.split_whitespace();
            let operation = match iter.next()? {
                "acc" => Operation::Acc,
                "jmp" => Operation::Jmp,
                "nop" => Operation::Nop,
                _ => return None,
            };

            let num = iter.next()?.parse::<i32>().ok()?;
            instructions.push((operation, num));
        }

        Some(Self { instructions })
    }

    fn part1(&self) -> u64 {
        let mut idx = 0i32;
        let mut acc = 0i32;
        let mut visited = vec![false; self.instructions.len()];

        while (idx as usize) < self.instructions.len() {
            if visited[idx as usize] {
                return acc as u64;
            }

            visited[idx as usize] = true;
            let (operation, num) = self.instructions[idx as usize];

            match operation {
                Operation::Acc => acc += num,
                Operation::Nop => (),
                Operation::Jmp => {
                    idx += num;
                    continue;
                }
            }
            idx += 1;
        }

        acc as u64
    }

    fn part2(&self) -> u64 {
        let mut try_correction = self.instructions.clone();
        let mut maybe_corrupted: Vec<usize> = self
            .instructions
            .iter()
            .enumerate()
            .filter(|(_, (operation, _))| {
                *operation == Operation::Jmp || *operation == Operation::Nop
            })
            .map(|(i, _)| i)
            .collect();

        'outer: while let Some(idx) = maybe_corrupted.pop() {
            try_correction[idx].0 = match self.instructions[idx].0 {
                Operation::Jmp => Operation::Nop,
                Operation::Nop => Operation::Jmp,
                Operation::Acc => Operation::Acc,
            };

            let mut acc = 0;
            let mut visited = vec![false; self.instructions.len()];
            let mut try_idx = 0;

            while (try_idx as usize) < self.instructions.len() {
                if visited[try_idx as usize] {
                    try_correction[idx].0 = self.instructions[idx].0;
                    continue 'outer;
                }

                visited[try_idx as usize] = true;
                let (operation, num) = try_correction[try_idx as usize];

                match operation {
                    Operation::Acc => acc += num,
                    Operation::Nop => (),
                    Operation::Jmp => {
                        try_idx += num;
                        continue;
                    }
                }

                try_idx += 1;
            }

            return acc as u64;
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn part1() {
        let input = include_str!("../example_input/day08.txt");
        let res = HandheldHalting::new_unwrap(input);
        assert_eq!(res.part1(), 5);
    }

    #[test]
    fn part2() {
        let input = include_str!("../example_input/day08.txt");
        let res = HandheldHalting::new_unwrap(input);
        assert_eq!(res.part2(), 8);
    }
}
