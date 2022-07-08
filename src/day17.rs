#[derive(PartialEq, Clone, Copy)]
enum State {
    Active,
    Inactive,
}

pub struct ConwayCubes {
    cube: Vec<Vec<State>>,
}

impl crate::AdventOfCode for ConwayCubes {
    const TITLE: &'static str = "Conway Cubes";
    const DAY: u8 = 17;

    fn new(input: &str) -> Option<Self> {
        let mut cube = Vec::new();

        for line in input.lines() {
            let row = line
                .chars()
                .map(|v| match v {
                    '.' => State::Inactive,
                    '#' => State::Active,
                    _ => unreachable!(),
                })
                .collect();

            cube.push(row);
        }

        Some(Self { cube })
    }

    fn part1(&self) -> u64 {
        self.cube.len() as u64
    }

    fn part2(&self) -> u64 {
        0
    }
}
