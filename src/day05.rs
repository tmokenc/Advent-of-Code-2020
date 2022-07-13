struct Ticket {
    row: u8,
    col: u8,
}

impl Ticket {
    fn id(&self) -> u64 {
        self.row as u64 * 8 + self.col as u64
    }
}
fn parse_row(s: &str) -> Option<u8> {
    let mut offset = 0;
    let mut avail = 128;

    for ch in s.chars() {
        avail /= 2;
        match ch {
            'B' => offset += avail,
            'F' => (),
            _ => return None,
        }
    }

    Some(offset)
}

fn parse_col(s: &str) -> Option<u8> {
    let mut offset = 0;
    let mut avail = 8;

    for ch in s.chars() {
        avail /= 2;
        match ch {
            'L' => (),
            'R' => offset += avail,
            _ => return None,
        }
    }

    Some(offset)
}

pub struct BinaryBoarding {
    tickets: Vec<Ticket>,
}

impl crate::AdventOfCode for BinaryBoarding {
    const TITLE: &'static str = "Binary Boarding";
    const DAY: u8 = 5;

    fn new(input: &str) -> Option<Self> {
        let mut tickets = Vec::new();

        for line in input.lines() {
            if line.len() != 10 {
                return None;
            }

            tickets.push(Ticket {
                row: parse_row(&line[0..7])?,
                col: parse_col(&line[7..])?,
            });
        }

        Some(Self { tickets })
    }

    fn part1(&self) -> u64 {
        self.tickets.iter().map(Ticket::id).max().unwrap_or(0)
    }

    fn part2(&self) -> u64 {
        let mut sorted_seat_ids: Vec<u64> = self.tickets.iter().map(Ticket::id).collect();
        sorted_seat_ids.sort_unstable();

        for i in 0..sorted_seat_ids.len() {
            if sorted_seat_ids[i + 1] - sorted_seat_ids[i] != 1 {
                return sorted_seat_ids[i] + 1;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn part1() {
        let input = include_str!("../example_input/day05.txt");
        let res = BinaryBoarding::new_unwrap(input);
        assert_eq!(res.part1(), 820);
    }

    // part 2 doesn't have an example for it
}
