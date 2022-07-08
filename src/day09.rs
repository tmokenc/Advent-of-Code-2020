pub struct EncodingError {
    list: Vec<u64>,
}

impl crate::AdventOfCode for EncodingError {
    const TITLE: &'static str = "Encoding Error";
    const DAY: u8 = 9;

    fn new(input: &str) -> Option<Self> {
        let mut list = Vec::new();

        for line in input.lines() {
            let num = line.parse::<u64>().ok()?;
            list.push(num);
        }

        Some(Self { list })
    }

    fn part1(&self) -> u64 {
        const PREAMBLE: usize = 25;
        let mut idx = PREAMBLE;

        while idx < self.list.len() {
            let num = self.list[idx];
            let preamble = &self.list[idx - PREAMBLE..idx];

            let has_some = preamble
                .iter()
                .take(PREAMBLE - 1)
                .enumerate()
                .flat_map(|(i, v)| preamble[i + 1..].into_iter().zip(std::iter::repeat(v)))
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
