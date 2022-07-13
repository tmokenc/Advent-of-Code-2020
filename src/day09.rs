pub struct EncodingError {
    list: Vec<u64>,
    preamble: usize,
}

impl crate::AdventOfCode for EncodingError {
    const TITLE: &'static str = "Encoding Error";
    const DAY: u8 = 9;

    fn new(input: &str) -> Option<Self> {
        Some(Self {
            list: crate::lines_to_vec::<u64>(input)?,
            preamble: 25,
        })
    }

    fn part1(&self) -> u64 {
        let mut idx = self.preamble;

        while idx < self.list.len() {
            let num = self.list[idx];
            let preamble = &self.list[idx - self.preamble..idx];

            let has_some = preamble
                .iter()
                .take(self.preamble - 1)
                .enumerate()
                .flat_map(|(i, v)| preamble[i + 1..].into_iter().map(move |x| (x, v)))
                .any(|(v, x)| v + x == num);

            if !has_some {
                return num;
            }

            idx += 1;
        }

        0
    }

    fn part2(&self) -> u64 {
        let invalid_num = self.part1();
        let invalid_num_idx = self
            .list
            .iter()
            .position(|v| *v == invalid_num)
            .unwrap_or(0);

        if invalid_num < 3 {
            return 0;
        }

        let mut set = Vec::with_capacity(2);

        'outer: for i in 0..(invalid_num_idx - 1) {
            let mut num = self.list[i];
            set = vec![num];

            for j in (i + 1)..invalid_num_idx {
                num += self.list[j];
                if num == invalid_num {
                    break 'outer;
                }

                set.push(self.list[j]);
            }
        }

        let mut min = u64::MAX;
        let mut max = u64::MIN;

        for num in set {
            if num < min {
                min = num;
            } else if num > max {
                max = num
            }
        }

        min + max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn part1() {
        let input = include_str!("../example_input/day09.txt");
        let mut res = EncodingError::new_unwrap(input);
        res.preamble = 5;
        assert_eq!(res.part1(), 127);
    }

    #[test]
    fn part2() {
        let input = include_str!("../example_input/day09.txt");
        let mut res = EncodingError::new_unwrap(input);
        res.preamble = 5;
        assert_eq!(res.part2(), 62);
    }
}
