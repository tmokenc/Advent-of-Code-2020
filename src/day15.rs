use std::collections::HashMap;

pub struct RambunctiousRecitation {
    nums: Vec<u64>,
}

impl RambunctiousRecitation {
    fn memory_game(&self, num_turn: u64) -> u64 {
        let mut data: HashMap<u64, u64> = HashMap::new();
        let mut last_spoken = *self.nums.last().unwrap();
        let next_turn = self.nums.len() as u64;

        data.extend(self.nums.iter().copied().zip(1..).take(self.nums.len() - 1));

        for turn in next_turn..num_turn {
            let old = last_spoken;
            last_spoken = data.get(&last_spoken).map(|v| turn - *v).unwrap_or(0);
            data.insert(old, turn);
        }

        last_spoken
    }
}

impl crate::AdventOfCode for RambunctiousRecitation {
    const TITLE: &'static str = "Rambunctious Recitation";
    const DAY: u8 = 15;

    fn new(input: &str) -> Option<Self> {
        Some(Self {
            nums: crate::string_to_vec(input, ",")?,
        })
    }

    fn part1(&self) -> u64 {
        self.memory_game(2020)
    }

    fn part2(&self) -> u64 {
        self.memory_game(30000000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn memory_game_1() {
        let input = "0,3,6";
        let res = RambunctiousRecitation::new_unwrap(input);
        assert_eq!(res.part1(), 436);
    }

    #[test]
    fn memory_game_2() {
        let input = "1,3,2";
        let res = RambunctiousRecitation::new_unwrap(input);
        assert_eq!(res.part1(), 1);
    }

    #[test]
    fn memory_game_3() {
        let input = "2,1,3";
        let res = RambunctiousRecitation::new_unwrap(input);
        assert_eq!(res.part1(), 10);
    }

    #[test]
    fn memory_game_4() {
        let input = "1,2,3";
        let res = RambunctiousRecitation::new_unwrap(input);
        assert_eq!(res.part1(), 27);
    }

    #[test]
    fn memory_game_5() {
        let input = "2,3,1";
        let res = RambunctiousRecitation::new_unwrap(input);
        assert_eq!(res.part1(), 78);
    }

    #[test]
    fn memory_game_6() {
        let input = "3,2,1";
        let res = RambunctiousRecitation::new_unwrap(input);
        assert_eq!(res.part1(), 438);
    }

    #[test]
    fn memory_game_7() {
        let input = "3,1,2";
        let res = RambunctiousRecitation::new_unwrap(input);
        assert_eq!(res.part1(), 1836);
    }
}
