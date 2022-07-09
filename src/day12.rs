type Coordinate = (isize, isize);

fn manhattan_distance(coord: (isize, isize)) -> u64 {
    coord.0.abs() as u64 + coord.1.abs() as u64
}

#[derive(Clone, Copy)]
enum Operation {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

pub struct RainRisk {
    actions: Vec<(Operation, u16)>,
}

impl crate::AdventOfCode for RainRisk {
    const TITLE: &'static str = "Rain Risk";
    const DAY: u8 = 12;

    fn new(input: &str) -> Option<Self> {
        let mut actions = Vec::new();

        for line in input.lines() {
            let num = line[1..].parse::<u16>().ok()?;
            let operation = match line.chars().next()? {
                'N' => Operation::North,
                'S' => Operation::South,
                'E' => Operation::East,
                'W' => Operation::West,
                'L' => Operation::Left,
                'R' => Operation::Right,
                'F' => Operation::Forward,
                _ => return None,
            };

            actions.push((operation, num));
        }

        Some(Self { actions })
    }

    fn part1(&self) -> u64 {
        let mut location: Coordinate = (0, 0);
        let mut facing = 90;

        for &(operation, num) in &self.actions {
            let num = num as isize;

            match operation {
                Operation::Forward => match facing {
                    0 => location.0 += num,
                    90 => location.1 += num,
                    180 => location.0 -= num,
                    270 => location.1 -= num,
                    _ => unreachable!(),
                },
                Operation::Left => facing = (facing + 360 - num) % 360,
                Operation::Right => facing = (facing + num) % 360,
                Operation::North => location.0 += num,
                Operation::East => location.1 += num,
                Operation::South => location.0 -= num,
                Operation::West => location.1 -= num,
            }
        }

        manhattan_distance(location)
    }

    fn part2(&self) -> u64 {
        let mut location: Coordinate = (0, 0);
        let mut waypoint: Coordinate = (1, 10); // distance from the ship

        for &(operation, num) in &self.actions {
            let num = num as isize;

            match operation {
                Operation::Forward => {
                    location.0 += num * waypoint.0;
                    location.1 += num * waypoint.1;
                }
                Operation::North => waypoint.0 += num,
                Operation::South => waypoint.0 -= num,
                Operation::East => waypoint.1 += num,
                Operation::West => waypoint.1 -= num,
                Operation::Left => {
                    let mut rotate = 0;
                    while rotate != num {
                        waypoint = (waypoint.1, -waypoint.0);
                        rotate += 90;
                    }
                }
                Operation::Right => {
                    let mut rotate = 0;
                    while rotate != num {
                        waypoint = (-waypoint.1, waypoint.0);
                        rotate += 90;
                    }
                }
            }
        }

        manhattan_distance(location)
    }
}
