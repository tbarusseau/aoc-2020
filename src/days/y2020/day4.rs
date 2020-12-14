use regex::Regex;

pub struct PassportBuilder {
    pub byr: Option<usize>,
    pub iyr: Option<usize>,
    pub eyr: Option<usize>,
    pub hgt: Option<String>,
    pub hcl: Option<String>,
    pub ecl: Option<String>,
    pub pid: Option<String>,
    pub cid: Option<String>,
}

impl PassportBuilder {
    pub fn new() -> PassportBuilder {
        PassportBuilder {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    pub fn build(&self) -> Result<Passport, Vec<PassportCreationError>> {
        let mut err = vec![];

        if self.byr.is_none() {
            err.push(PassportCreationError::BirthYearMissing);
        }
        if self.iyr.is_none() {
            err.push(PassportCreationError::IssueYearMissing);
        }
        if self.eyr.is_none() {
            err.push(PassportCreationError::ExpirationYearMissing);
        }
        if self.hgt.is_none() {
            err.push(PassportCreationError::HeightMissing);
        }
        if self.hcl.is_none() {
            err.push(PassportCreationError::HairColorMissing);
        }
        if self.ecl.is_none() {
            err.push(PassportCreationError::EyeColorMissing);
        }
        if self.pid.is_none() {
            err.push(PassportCreationError::PassportIdMissing);
        }

        // Validation
        if let Some(byr) = self.byr {
            if !(1920..=2002).contains(&byr) {
                err.push(PassportCreationError::InvalidBirthYear);
            }
        }

        if let Some(iyr) = self.iyr {
            if !(2010..=2020).contains(&iyr) {
                err.push(PassportCreationError::InvalidIssueYear);
            }
        }

        if let Some(eyr) = self.eyr {
            if !(2020..=2030).contains(&eyr) {
                err.push(PassportCreationError::InvalidExpirationYear);
            }
        }

        if let Some(hgt) = &self.hgt {
            if hgt.ends_with("in") {
                let v: usize = hgt[..hgt.len() - 2].parse().expect("Invalid 'in' height");
                if !(59..=76).contains(&v) {
                    err.push(PassportCreationError::InvalidHeight);
                }
            } else if hgt.ends_with("cm") {
                let v: usize = hgt[..hgt.len() - 2].parse().expect("Invalid 'cm' height");
                if !(150..=193).contains(&v) {
                    err.push(PassportCreationError::InvalidHeight);
                }
            } else {
                err.push(PassportCreationError::InvalidHeight);
            }
        }

        if let Some(hcl) = &self.hcl {
            let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            if !re.is_match(&hcl) {
                err.push(PassportCreationError::InvalidHairColor);
            }
        }

        if let Some(ecl) = &self.ecl {
            if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&ecl[..]) {
                err.push(PassportCreationError::InvalidEyeColor);
            }
        }

        if let Some(pid) = &self.pid {
            let re = Regex::new(r"^\d{9}$").unwrap();
            if !re.is_match(&pid) {
                err.push(PassportCreationError::InvalidPassportId);
            }
        }

        if !err.is_empty() {
            return Err(err);
        }

        Ok(Passport {
            byr: self.byr.unwrap(),
            iyr: self.iyr.unwrap(),
            eyr: self.eyr.unwrap(),
            hgt: self.hgt.clone().unwrap(),
            hcl: self.hcl.clone().unwrap(),
            ecl: self.ecl.clone().unwrap(),
            pid: self.pid.clone().unwrap(),
            cid: self.cid.clone(),
        })
    }
}

pub struct Passport {
    pub byr: usize,          // Birth Year
    pub iyr: usize,          // Issue Year
    pub eyr: usize,          // Expiration Year
    pub hgt: String,         // Height
    pub hcl: String,         // Hair Color
    pub ecl: String,         // Eye Color
    pub pid: String,         // Passport ID
    pub cid: Option<String>, // Country ID
}

#[derive(Debug)]
pub enum PassportCreationError {
    BirthYearMissing,
    IssueYearMissing,
    ExpirationYearMissing,
    HeightMissing,
    HairColorMissing,
    EyeColorMissing,
    PassportIdMissing,
    InvalidBirthYear,
    InvalidIssueYear,
    InvalidExpirationYear,
    InvalidHeight,
    InvalidHairColor,
    InvalidEyeColor,
    InvalidPassportId,
}

impl Passport {
    pub fn from(input: &str) -> Result<Passport, Vec<PassportCreationError>> {
        let mut pb = PassportBuilder::new();
        input[..input.len() - 1].split(' ').for_each(|w| {
            let mut split = w.split(':');
            let key = split.next().expect("Couldn't get key");
            let value = split.next().expect("Couldn't get value");

            match key {
                "byr" => pb.byr = Some(value.parse().expect("Invalid birth year")),
                "iyr" => pb.iyr = Some(value.parse().expect("Invalid issue year")),
                "eyr" => pb.eyr = Some(value.parse().expect("Invalid expiration year")),
                "hgt" => pb.hgt = Some(value.to_string()),
                "hcl" => pb.hcl = Some(value.to_string()),
                "ecl" => pb.ecl = Some(value.to_string()),
                "pid" => pb.pid = Some(value.to_string()),
                "cid" => pb.cid = Some(value.to_string()),
                _ => panic!("Invalid key: {}", key),
            }
        });

        pb.build()
    }
}

fn generate_input(input: &str) -> Vec<Result<Passport, Vec<PassportCreationError>>> {
    let mut v: Vec<Result<Passport, Vec<PassportCreationError>>> = vec![];
    let mut s = String::new();

    input.lines().for_each(|l| {
        if l.is_empty() {
            v.push(Passport::from(&s));
            s.clear();
        } else {
            s.push_str(&format!("{} ", l));
        }
    });

    v
}

pub fn solve_part1(input: &str) -> usize {
    let input = generate_input(input);

    input.iter().filter_map(|i| Result::ok(i.as_ref())).count()
}

pub fn solve_part2(input: &str) -> usize {
    let input = generate_input(input);

    input.iter().filter_map(|i| Result::ok(i.as_ref())).count()
}
