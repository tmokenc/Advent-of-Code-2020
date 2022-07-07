use std::collections::HashMap;

pub struct HandyHaversacks {
    list: HashMap<String, Vec<(String, u8)>>,
}

impl HandyHaversacks {
    fn get_bag_size(&self, bag: &str) -> u64 {
        let mut count = 0;

        for (subbag, num) in self.list.get(bag).into_iter().flatten() {
            count += *num as u64;
            count += self.get_bag_size(subbag) * *num as u64;
        }

        count
    }
}

impl crate::AdventOfCode for HandyHaversacks {
    const TITLE: &'static str = "Handy Haversacks";
    const DAY: u8 = 7;

    fn new(input: &str) -> Option<Self> {
        let mut list = HashMap::new();

        for line in input.lines() {
            let mut iter = line.split_whitespace();
            let bag = format!("{} {}", iter.next()?, iter.next()?);
            let mut items = Vec::new();

            iter.next()?;
            iter.next()?;

            while let Ok(num) = iter.next()?.parse::<u8>() {
                let bag = format!("{} {}", iter.next()?, iter.next()?);
                items.push((bag, num));

                if matches!(iter.next(), Some(v) if v.ends_with('.')) {
                    break;
                }
            }

            list.insert(bag, items);
        }

        log::debug!("{:#?}", &list);

        Some(Self { list })
    }

    fn part1(&self) -> u64 {
        let mut count = 0;

        'outer: for holding in self.list.values() {
            let mut checking = Vec::with_capacity(holding.len());

            for (subbag, _) in holding {
                if subbag == "shiny gold" {
                    count += 1;
                    continue 'outer;
                }

                checking.push(subbag.as_str());
            }

            'l: while let Some(subbag) = checking.pop() {
                for (sbag, _) in self.list.get(subbag).into_iter().flatten() {
                    if sbag == "shiny gold" {
                        count += 1;
                        break 'l;
                    }

                    checking.push(&sbag);
                }
            }
        }

        count
    }

    fn part2(&self) -> u64 {
        self.get_bag_size("shiny gold")
    }
}
