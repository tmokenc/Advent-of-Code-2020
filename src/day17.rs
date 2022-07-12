use std::collections::HashSet;
use std::mem;

type Coord = (isize, isize, isize, isize);

const OFFSET: [isize; 3] = [-1, 0, 1];

#[derive(Default, Clone)]
pub struct ConwayCubes {
    active_cubes: HashSet<Coord>,
    min: Coord,
    max: Coord,
}

impl ConwayCubes {
    fn count_neighbor_active_3d(&self, (x, y, z, w): Coord) -> usize {
        OFFSET
            .into_iter()
            .flat_map(|x| OFFSET.map(|y| (x, y)))
            .flat_map(|(x, y)| OFFSET.map(|z| (x, y, z, 0)))
            .filter(|&v| v != (0, 0, 0, 0))
            // generated offet iterator
            .map(|(dx, dy, dz, _)| (x + dx, y + dy, z + dz, w))
            .filter(|&coord| self.is_active(coord))
            .count()
    }

    fn count_neighbor_active(&self, (x, y, z, w): Coord) -> usize {
        OFFSET
            .into_iter()
            .flat_map(|x| OFFSET.map(|y| (x, y)))
            .flat_map(|(x, y)| OFFSET.map(|z| (x, y, z)))
            .flat_map(|(x, y, z)| OFFSET.map(|w| (x, y, z, w)))
            .filter(|&v| v != (0, 0, 0, 0))
            // generated offet iterator
            .map(|(dx, dy, dz, dw)| (x + dx, y + dy, z + dz, w + dw))
            .filter(|&coord| self.is_active(coord))
            .count()
    }

    fn iter_3d(&self) -> impl Iterator<Item = Coord> {
        let x = (self.min.0 - 1)..=(self.max.0 + 1);
        let y = (self.min.1 - 1)..=(self.max.1 + 1);
        let z = (self.min.2 - 1)..=(self.max.2 + 1);

        x.flat_map(move |x| y.clone().map(move |y| (x, y)))
            .flat_map(move |(x, y)| z.clone().map(move |z| (x, y, z, 0)))
    }

    fn iter(&self) -> impl Iterator<Item = Coord> {
        let x = (self.min.0 - 1)..=(self.max.0 + 1);
        let y = (self.min.1 - 1)..=(self.max.1 + 1);
        let z = (self.min.2 - 1)..=(self.max.2 + 1);
        let w = (self.min.3 - 1)..=(self.max.3 + 1);

        x.flat_map(move |x| y.clone().map(move |y| (x, y)))
            .flat_map(move |(x, y)| z.clone().map(move |z| (x, y, z)))
            .flat_map(move |(x, y, z)| w.clone().map(move |w| (x, y, z, w)))
    }

    fn set_active(&mut self, coord: Coord) {
        self.active_cubes.insert(coord);

        self.min.0 = self.min.0.min(coord.0);
        self.min.1 = self.min.1.min(coord.1);
        self.min.2 = self.min.2.min(coord.2);
        self.min.3 = self.min.3.min(coord.3);

        self.max.0 = self.max.0.max(coord.0);
        self.max.1 = self.max.1.max(coord.1);
        self.max.2 = self.max.2.max(coord.2);
        self.max.3 = self.max.3.max(coord.3);
    }

    fn is_active(&self, coord: Coord) -> bool {
        self.active_cubes.contains(&coord)
    }

    fn cycle_3d(&mut self) {
        let old = mem::take(self);

        for coord in old.iter_3d() {
            let neighbor_actives = old.count_neighbor_active_3d(coord);

            let cond1 = old.is_active(coord) && matches!(neighbor_actives, 2 | 3);
            let cond2 = !old.is_active(coord) && neighbor_actives == 3;

            if cond1 || cond2 {
                self.set_active(coord);
            }
        }
    }

    fn cycle(&mut self) {
        let old = mem::take(self);

        for coord in old.iter() {
            let neighbor_actives = old.count_neighbor_active(coord);
            let cond1 = old.is_active(coord) && matches!(neighbor_actives, 2 | 3);
            let cond2 = !old.is_active(coord) && neighbor_actives == 3;

            if cond1 || cond2 {
                self.set_active(coord);
            }
        }
    }
}

impl crate::AdventOfCode for ConwayCubes {
    const TITLE: &'static str = "Conway Cubes";
    const DAY: u8 = 17;

    fn new(input: &str) -> Option<Self> {
        let iter = input.lines().zip(0..).flat_map(|(line, x)| {
            line.chars()
                .zip(0..)
                .filter(|(v, _)| *v == '#')
                .map(move |(_, y)| (x, y, 0, 0))
        });

        let mut active_cubes = HashSet::new();
        let mut max = (0, 0, 0, 0);

        for coord in iter {
            max.0 = coord.0.max(max.0);
            max.1 = coord.1.max(max.1);
            active_cubes.insert(coord);
        }

        Some(Self {
            active_cubes,
            max,
            min: (0, 0, 0, 0),
        })
    }

    fn part1(&self) -> u64 {
        let mut cubes = self.clone();
        (0..6).for_each(|_| cubes.cycle_3d());
        cubes.active_cubes.len() as u64
    }

    fn part2(&self) -> u64 {
        let mut cubes = self.clone();
        (0..6).for_each(|_| cubes.cycle());
        cubes.active_cubes.len() as u64
    }
}
