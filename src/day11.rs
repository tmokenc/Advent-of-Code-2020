const DIRECTION: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(PartialEq, Clone, Copy)]
enum Seat {
    Empty,
    Floor,
    Occupied,
}

#[derive(PartialEq, Clone)]
pub struct SeatingSystem {
    seats: Vec<Vec<Seat>>,
}

impl SeatingSystem {
    fn get(&self, (y, x): (usize, usize)) -> Seat {
        self.seats[y][x]
    }

    fn get_adjacent_seats(&self, (y, x): (usize, usize)) -> [Seat; 8] {
        let mut seats = [Seat::Floor; 8];
        let y_len = self.seats.len();
        let x_len = self.seats[0].len();

        for (i, (a, b)) in DIRECTION.into_iter().enumerate() {
            let new_y = usize::try_from(y as isize + a).ok().filter(|&v| v < y_len);
            let new_x = usize::try_from(x as isize + b).ok().filter(|&v| v < x_len);

            if let Some(coor) = new_y.zip(new_x) {
                seats[i] = self.get(coor);
            }
        }

        seats
    }

    fn get_actual_adjacent_seats(&self, (y, x): (usize, usize)) -> [Seat; 8] {
        let mut seats = [Seat::Floor; 8];
        let y_len = self.seats.len();
        let x_len = self.seats[0].len();

        for (i, (a, b)) in DIRECTION.into_iter().enumerate() {
            let mut new_y = usize::try_from(y as isize + a).ok().filter(|&v| v < y_len);
            let mut new_x = usize::try_from(x as isize + b).ok().filter(|&v| v < x_len);

            while let Some(coor) = new_y.zip(new_x) {
                seats[i] = self.get(coor);

                if self.get(coor) != Seat::Floor {
                    break;
                }

                new_y = usize::try_from(coor.0 as isize + a)
                    .ok()
                    .filter(|&v| v < y_len);
                new_x = usize::try_from(coor.1 as isize + b)
                    .ok()
                    .filter(|&v| v < x_len);
            }
        }

        seats
    }

    fn apply_round(&mut self) {
        let mut new = self.clone();

        for y in 0..new.seats.len() {
            for x in 0..new.seats[0].len() {
                let coor = (y, x);

                if self.get(coor) == Seat::Floor {
                    continue;
                }

                let data = self.get_adjacent_seats(coor);
                let count_occupied = data.into_iter().filter(|v| *v == Seat::Occupied).count();

                new.seats[y][x] = match self.get(coor) {
                    Seat::Empty if count_occupied == 0 => Seat::Occupied,
                    Seat::Occupied if count_occupied >= 4 => Seat::Empty,
                    v => v,
                };
            }
        }

        *self = new;
    }

    fn apply_actual_round(&mut self) {
        let mut new = self.clone();

        for y in 0..new.seats.len() {
            for x in 0..new.seats[0].len() {
                let coor = (y, x);

                if self.get(coor) == Seat::Floor {
                    continue;
                }

                let data = self.get_actual_adjacent_seats(coor);
                let count_occupied = data.into_iter().filter(|v| *v == Seat::Occupied).count();

                new.seats[y][x] = match self.get(coor) {
                    Seat::Empty if count_occupied == 0 => Seat::Occupied,
                    Seat::Occupied if count_occupied >= 5 => Seat::Empty,
                    v => v,
                };
            }
        }

        *self = new;
    }
    fn count_occupied(&self) -> u64 {
        self.seats
            .iter()
            .flatten()
            .filter(|&&v| v == Seat::Occupied)
            .count() as u64
    }
}

impl crate::AdventOfCode for SeatingSystem {
    const TITLE: &'static str = "Seating System";
    const DAY: u8 = 11;

    fn new(input: &str) -> Option<Self> {
        let mut seats = Vec::new();

        for line in input.lines() {
            let mut row = Vec::new();

            for ch in line.chars() {
                row.push(match ch {
                    'L' => Seat::Empty,
                    '.' => Seat::Floor,
                    '#' => Seat::Occupied,
                    _ => return None,
                });
            }

            seats.push(row);
        }

        Some(Self { seats })
    }

    fn part1(&self) -> u64 {
        let mut seats = self.clone();
        let mut new = seats.clone();
        new.apply_round();

        while new != seats {
            seats = new.clone();
            new.apply_round();
        }

        seats.count_occupied()
    }

    fn part2(&self) -> u64 {
        let mut seats = self.clone();
        let mut new = seats.clone();
        new.apply_actual_round();

        while new != seats {
            seats = new.clone();
            new.apply_actual_round();
        }

        seats.count_occupied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn part1() {
        let input = include_str!("../example_input/day11.txt");
        let res = SeatingSystem::new_unwrap(input);
        assert_eq!(res.part1(), 37);
    }

    #[test]
    fn part2() {
        let input = include_str!("../example_input/day11.txt");
        let res = SeatingSystem::new_unwrap(input);
        assert_eq!(res.part2(), 26);
    }
}
