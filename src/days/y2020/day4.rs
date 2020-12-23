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

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        assert_eq!(
            solve_part1(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
            ),
            2
        );
    }

    #[test]
    pub fn test_passport_parsing() {
        assert_eq!(
            solve_part2(
                //                 "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n
                // iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946\n
                // hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n
                // hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007\n
                "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f\n
eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n
hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022\n
iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
            ),
            4
        )
    }
}
