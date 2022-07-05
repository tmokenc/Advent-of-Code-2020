#[derive(Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

fn check_num(s: Option<&str>, min: u64, max: u64, len: usize) -> bool {
    let s = match s {
        Some(s) => s,
        None => return false,
    };

    if s.len() != len {
        return false;
    }

    s.parse::<u64>()
        .ok()
        .filter(|&num| num >= min && num <= max)
        .is_some()
}

fn check_height(s: Option<&str>) -> bool {
    let s = match s {
        Some(s) => s,
        None => return false,
    };

    if s.len() < 4 && s.len() > 6 {
        return false;
    }

    let split_idx = s.len() - 2;
    match &s[split_idx..] {
        "cm" => check_num(Some(&s[..split_idx]), 150, 193, 3),
        "in" => check_num(Some(&s[..split_idx]), 59, 76, 2),
        _ => false,
    }
}

fn check_hair_color(s: Option<&str>) -> bool {
    let s = match s {
        Some(s) => s,
        None => return false,
    };

    let mut chars = s.chars();
    if chars.next() != Some('#') {
        return false;
    }

    let mut count_num = 0;

    for ch in chars.by_ref().take(6) {
        if ch.is_ascii_uppercase() || !ch.is_alphanumeric() {
            return false;
        }

        count_num += 1;
    }

    count_num == 6 && chars.next().is_none()
}

impl Passport {
    fn is_valid(&self) -> bool {
        check_num(self.byr.as_deref(), 1920, 2002, 4)
            && check_num(self.iyr.as_deref(), 2010, 2020, 4)
            && check_num(self.eyr.as_deref(), 2020, 2030, 4)
            && check_height(self.hgt.as_deref())
            && check_hair_color(self.hcl.as_deref())
            && matches!(
                self.ecl.as_deref(),
                Some("amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
            )
            && check_num(self.pid.as_deref(), u64::MIN, u64::MAX, 9)
    }
}

pub struct PassportProcessing {
    passports: Vec<Passport>,
}

impl crate::AdventOfCode for PassportProcessing {
    const TITLE: &'static str = "Passport Processing";
    const DAY: u8 = 4;

    fn new(input: &str) -> Option<Self> {
        let mut passports = Vec::new();

        for passport_data in input.split("\n\n") {
            let mut passport = Passport::default();

            for data in passport_data.split_whitespace() {
                let mut iter = data.split(':');
                let typ = iter.next()?;
                let value = iter.next()?.to_owned();

                match typ {
                    "byr" => passport.byr = Some(value),
                    "iyr" => passport.iyr = Some(value),
                    "eyr" => passport.eyr = Some(value),
                    "hgt" => passport.hgt = Some(value),
                    "hcl" => passport.hcl = Some(value),
                    "ecl" => passport.ecl = Some(value),
                    "pid" => passport.pid = Some(value),
                    "cid" => passport.cid = Some(value),
                    _ => return None,
                }
            }

            passports.push(passport);
        }

        Some(Self { passports })
    }

    fn part1(&self) -> u64 {
        self.passports
            .iter()
            .filter(|v| {
                v.byr.is_some()
                    && v.iyr.is_some()
                    && v.eyr.is_some()
                    && v.hgt.is_some()
                    && v.hcl.is_some()
                    && v.ecl.is_some()
                    && v.pid.is_some()
            })
            .count() as u64
    }

    fn part2(&self) -> u64 {
        self.passports.iter().filter(|v| v.is_valid()).count() as u64
    }
}
