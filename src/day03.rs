#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Open,
    Tree,
}

pub struct TobogganTrajectory {
    map: Vec<Vec<Tile>>,
}

impl TobogganTrajectory {
    fn count_tree_encountered(&self, x: usize, y: usize) -> u64 {
        let mut count = 0;
        let mut pos_x = x;
        let mut pos_y = y;

        while pos_y < self.map.len() {
            if self.map[pos_y][pos_x] == Tile::Tree {
                count += 1;
            }

            pos_x += x;
            pos_y += y;

            if pos_x >= self.map[0].len() {
                pos_x %= self.map[0].len();
            }
        }

        count
    }
}

impl crate::AdventOfCode for TobogganTrajectory {
    const TITLE: &'static str = "Toboggan Trajectory";
    const DAY: u8 = 3;

    fn new(input: &str) -> Option<Self> {
        let mut map = Vec::new();

        for line in input.lines() {
            let row = line
                .chars()
                .map(|ch| match ch {
                    '.' => Tile::Open,
                    '#' => Tile::Tree,
                    _ => unreachable!(),
                })
                .collect();

            map.push(row);
        }

        Some(Self { map })
    }

    fn part1(&self) -> u64 {
        self.count_tree_encountered(3, 1)
    }

    fn part2(&self) -> u64 {
        let a = self.count_tree_encountered(1, 1);
        let b = self.count_tree_encountered(3, 1);
        let c = self.count_tree_encountered(5, 1);
        let d = self.count_tree_encountered(7, 1);
        let e = self.count_tree_encountered(1, 2);

        a * b * c * d * e
    }
}
