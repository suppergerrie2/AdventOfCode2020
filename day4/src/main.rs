use aoc_utils::day_runner::run_day;
use aoc_utils::input_helper::read_vector_from_file;
use regex::Regex;

#[derive(Debug)]
struct Passport {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

fn main() {
    run_day(part1, part2);
}

fn part1() {
    let passports = load_passports();
    let passports: Vec<&Passport> = passports.iter().filter(contains_needed_information).collect();

    println!("Found {} valid passports", passports.len())
}

fn part2() {
    let passports = load_passports();
    let passports: Vec<&Passport> = passports.iter().filter(contains_needed_information).filter(is_valid_passport).collect();

    println!("Found {} valid passports", passports.len())
}

fn contains_needed_information(passport: &&Passport) -> bool {
    passport.byr.is_some() &&
        passport.iyr.is_some() &&
        passport.eyr.is_some() &&
        passport.hgt.is_some() &&
        passport.hcl.is_some() &&
        passport.ecl.is_some() &&
        passport.pid.is_some()
}

fn is_valid_passport(passport: &&Passport) -> bool {
    let byr = passport.byr.unwrap();
    let iyr = passport.iyr.unwrap();
    let eyr = passport.eyr.unwrap();
    let hgt = passport.hgt.as_ref().unwrap();
    let hcl = passport.hcl.as_ref().unwrap();
    let ecl = passport.ecl.as_ref().unwrap();
    let pid = passport.pid.as_ref().unwrap();

    let mut height_valid = false;

    if hgt.ends_with("cm") {
        let h: u64 = hgt.replace("cm", "").parse().unwrap_or(0);

        height_valid = h >= 150 && h <= 193;
    } else if hgt.ends_with("in") {
        let h: u64 = hgt.replace("in", "").parse().unwrap_or(0);

        height_valid = h >= 59 && h <= 76;
    }

    let color_regex = Regex::new(r"^#(?:[0-9a-fA-F]{6})$").unwrap();
    let pid_regex = Regex::new(r"^(?:[0-9]{9})$").unwrap();

    byr >= 1920 && byr <= 2002 &&
        iyr >= 2010 && iyr <= 2020 &&
        eyr >= 2020 && eyr <= 2030 &&
        height_valid &&
        color_regex.is_match(hcl) &&
        (ecl.eq("amb") || ecl.eq("blu") || ecl.eq("brn") || ecl.eq("gry") || ecl.eq("grn") ||ecl.eq("hzl") ||ecl.eq("oth")) &&
        pid_regex.is_match(pid)
}

fn load_passports() -> Vec<Passport> {
    let passwords: Vec<String> = read_vector_from_file("../input/day4/input.txt", "\r\n\r\n").expect("Failed to load input");

    passwords.iter().map(parse_passport).collect()
}

fn parse_passport(string: &String) -> Passport {
    let s : Vec<String> = string.split_whitespace().map(str::to_string).collect();
    let mut passport = Passport{
        byr: None,
        iyr: None,
        eyr: None,
        hgt: None,
        hcl: None,
        ecl: None,
        pid: None,
        cid: None
    };

    for s in s {
        let key_value : Vec<&str> = s.split(":").collect();
        match key_value[0] {
            "byr" => passport.byr = Some(key_value[1].parse().expect("failed to parse byr")),
            "iyr" => passport.iyr = Some(key_value[1].parse().expect("failed to parse iyr")),
            "eyr" => passport.eyr = Some(key_value[1].parse().expect("failed to parse eyr")),
            "hgt" => passport.hgt = Some(key_value[1].to_string()),
            "hcl" => passport.hcl = Some(key_value[1].to_string()),
            "ecl" => passport.ecl = Some(key_value[1].to_string()),
            "pid" => passport.pid = Some(key_value[1].to_string()),
            "cid" => passport.cid = Some(key_value[1].to_string()),

            &_ => {}
        }
    }

    passport
}