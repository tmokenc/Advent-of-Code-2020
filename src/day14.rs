use std::collections::HashMap;

#[derive(Clone, Copy)]
enum MaskValue {
    Zero,
    One,
    X,
}

#[derive(Clone, Copy)]
struct Mask([MaskValue; 36]);

impl Default for Mask {
    fn default() -> Self {
        Self([MaskValue::X; 36])
    }
}

impl Mask {
    fn parse(s: &str) -> Option<Self> {
        if s.len() != 36 {
            return None;
        }

        let mut mask = [MaskValue::X; 36];

        for (i, ch) in s.chars().enumerate() {
            mask[i] = match ch {
                'X' => MaskValue::X,
                '1' => MaskValue::One,
                '0' => MaskValue::Zero,
                _ => return None,
            };
        }

        Some(Self(mask))
    }

    fn apply(&self, mut num: u64) -> u64 {
        for (i, mask) in self.0.into_iter().rev().enumerate() {
            let value = match mask {
                MaskValue::Zero => 0,
                MaskValue::One => 1,
                MaskValue::X => continue,
            };

            let m = 1 << i;
            num = (num & !m) | (value << i);
        }

        num
    }

    fn decode_memory_address(&self, num: u64) -> Vec<u64> {
        let mut addresses = vec![num];

        for (i, mask) in self.0.into_iter().rev().enumerate() {
            if let MaskValue::Zero = mask {
                continue;
            }

            let m = 1 << i;
            let mut tmp = Vec::new();

            for address in &mut addresses {
                *address &= !m;
                *address |= 1 << i;

                if let MaskValue::X = mask {
                    tmp.push(*address & !m);
                }
            }

            addresses.extend(tmp);
        }

        addresses
    }
}

#[derive(Clone, Copy)]
enum Operation {
    Mask(Mask),
    Insert(u64, u64),
}

pub struct DockingData {
    operations: Vec<Operation>,
}

impl crate::AdventOfCode for DockingData {
    const TITLE: &'static str = "Docking Data";
    const DAY: u8 = 14;

    fn new(input: &str) -> Option<Self> {
        let mut operations = Vec::new();

        for line in input.lines() {
            if line.starts_with("mask = ") {
                let mask = Mask::parse(&line["mask = ".len()..])?;
                operations.push(Operation::Mask(mask));
                continue;
            }

            let mut iter = line["mem[".len()..].split("] = ");
            let key = iter.next()?.parse::<u64>().ok()?;
            let val = iter.last()?.parse::<u64>().ok()?;
            operations.push(Operation::Insert(key, val));
        }

        Some(Self { operations })
    }

    fn part1(&self) -> u64 {
        let mut mask = Mask::default();
        let mut data = HashMap::new();

        for &operation in &self.operations {
            match operation {
                Operation::Mask(m) => mask = m,
                Operation::Insert(idx, num) => {
                    let val = mask.apply(num);
                    data.insert(idx, val);
                }
            }
        }

        data.values().copied().sum()
    }

    fn part2(&self) -> u64 {
        let mut mask = Mask::default();
        let mut data = HashMap::new();

        for &operation in &self.operations {
            match operation {
                Operation::Mask(m) => mask = m,
                Operation::Insert(idx, num) => {
                    for address in mask.decode_memory_address(idx) {
                        data.insert(address, num);
                    }
                }
            }
        }

        data.values().copied().sum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn part1() {
        let input = include_str!("../example_input/day14.txt");
        let res = DockingData::new_unwrap(input);
        assert_eq!(res.part1(), 165);
    }

    #[test]
    fn part2() {
        let input = include_str!("../example_input/day14_2.txt");
        let res = DockingData::new_unwrap(input);
        assert_eq!(res.part2(), 208);
    }
}
