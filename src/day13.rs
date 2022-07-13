pub struct ShuttleSearch {
    timestamp: u64,
    buses: Vec<Option<u64>>,
}

impl crate::AdventOfCode for ShuttleSearch {
    const TITLE: &'static str = "Shuttle Search";
    const DAY: u8 = 13;

    fn new(input: &str) -> Option<Self> {
        let mut iter = input.lines();
        let timestamp = iter.next()?.parse::<u64>().ok()?;
        let buses = iter
            .next()?
            .split(',')
            .map(|v| v.parse::<u64>().ok())
            .collect();

        Some(Self { timestamp, buses })
    }

    fn part1(&self) -> u64 {
        self.buses
            .iter()
            .filter_map(|v| {
                let val = v.as_ref().copied()?;
                let loops = self.timestamp as f64 / val as f64;
                let loops = loops.ceil() as u64;
                let next = val * loops - self.timestamp;
                Some((val, next))
            })
            .min_by_key(|v| v.1)
            .map(|(v, next)| v * next)
            .unwrap_or(0)
    }

    fn part2(&self) -> u64 {
        let data: Vec<(u64, u64)> = self
            .buses
            .iter()
            .zip(0..)
            .filter_map(|(&v, i)| v.zip(Some(i)))
            .collect();

        let mut timestamp = 0;
        let mut inc = 1;

        for (bus, offset) in data {
            while (timestamp + offset) % bus != 0 {
                timestamp += inc;
            }

            inc *= bus;
        }

        timestamp
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn part1() {
        let input = include_str!("../example_input/day13.txt");
        let res = ShuttleSearch::new_unwrap(input);
        assert_eq!(res.part1(), 295);
    }

    #[test]
    fn part2() {
        let input = include_str!("../example_input/day13.txt");
        let res = ShuttleSearch::new_unwrap(input);
        assert_eq!(res.part2(), 1068781);
    }
}
