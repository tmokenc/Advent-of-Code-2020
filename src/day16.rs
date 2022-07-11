use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

type Ticket = Vec<u64>;
type TicketData = HashMap<String, TicketValue>;

#[derive(Debug)]
struct TicketValue(RangeInclusive<u64>, RangeInclusive<u64>);

impl TicketValue {
    fn matches(&self, val: u64) -> bool {
        self.0.contains(&val) || self.1.contains(&val)
    }
}

fn parse_data(s: &str) -> Option<TicketData> {
    let mut data = HashMap::new();

    for line in s.lines() {
        let mut iter = line.split(": ");
        let key = iter.next()?.to_owned();
        let mut values = iter
            .next()?
            .split(" or ")
            .flat_map(|v| v.trim().split('-'))
            .filter_map(|v| v.parse::<u64>().ok());

        let x = RangeInclusive::new(values.next()?, values.next()?);
        let y = RangeInclusive::new(values.next()?, values.next()?);

        data.insert(key, TicketValue(x, y));
    }

    Some(data)
}

fn parse_ticket(s: &str) -> Option<Ticket> {
    crate::string_to_vec::<u64>(s, ",")
}

pub struct TicketTranslation {
    data: TicketData,
    ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl crate::AdventOfCode for TicketTranslation {
    const TITLE: &'static str = "Ticket Translation";
    const DAY: u8 = 16;

    fn new(input: &str) -> Option<Self> {
        let mut iter = input.split("\n\n");

        let data = parse_data(iter.next()?)?;
        let ticket = parse_ticket(iter.next()?.lines().nth(1)?)?;
        let nearby_tickets = iter
            .next()?
            .lines()
            .skip(1)
            .filter_map(parse_ticket)
            .collect();

        Some(Self {
            data,
            ticket,
            nearby_tickets,
        })
    }

    fn part1(&self) -> u64 {
        self.nearby_tickets
            .iter()
            .flatten()
            .filter(|v| {
                !self
                    .data
                    .values()
                    .any(|x| x.0.contains(v) || x.1.contains(v))
            })
            .sum()
    }

    fn part2(&self) -> u64 {
        let valid_tickets = self
            .nearby_tickets
            .iter()
            .filter(|v| v.iter().all(|x| self.data.values().any(|y| y.matches(*x))))
            .collect::<Vec<_>>();

        let mut indexes = vec![HashSet::<&str>::new(); self.data.len()];

        for (i, value) in valid_tickets[0].iter().copied().enumerate() {
            for (key, val) in &self.data {
                if !val.matches(value) {
                    continue;
                }

                if valid_tickets[1..].iter().all(|v| val.matches(v[i])) {
                    indexes[i].insert(key);
                }
            }
        }

        let mut exact: Vec<&str> = indexes
            .iter()
            .filter(|v| v.len() == 1)
            .map(|v| *v.iter().next().unwrap())
            .collect();

        while exact.len() < indexes.len() {
            for &val in &exact {
                for idx in indexes.iter_mut() {
                    if idx.len() > 1 {
                        idx.remove(val);
                    }
                }
            }

            exact = indexes
                .iter()
                .filter(|v| v.len() == 1)
                .map(|v| *v.iter().next().unwrap())
                .collect();
        }

        indexes
            .into_iter()
            .enumerate()
            .filter(|(_, v)| v.into_iter().next().unwrap().starts_with("departure"))
            .map(|(i, _)| self.ticket[i])
            .product()
    }
}
