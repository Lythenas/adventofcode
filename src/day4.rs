use crate::utils::read_lines;

pub fn day4() {
    let kv_pairs: Vec<String> = read_lines("day4.input")
        .flat_map(|line| line.split(" ").map(|x| x.to_string()).collect::<Vec<_>>())
        .collect();
    println!("Part 1: {}", part1(&kv_pairs));
    println!("Part 2: {}", part2(&kv_pairs));
}

fn part1(kv_pairs: &[String]) -> usize {
    let mut fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]; // , "cid"
    fields.sort();

    let mut count_valid = 0;

    let mut passport = Vec::new();
    for pair in kv_pairs {
        if pair.is_empty() {
            // check if complete and increment count
            passport.sort();
            if fields == passport[..] {
                count_valid += 1;
            }
            passport.clear();
        } else {
            let mut split = pair.split(":");
            let key = split.next().unwrap();

            if key != "cid" {
                passport.push(key);
            }
        }
    }

    // also check the last passport (if there is no newline after it)
    passport.sort();
    if fields == passport[..] {
        count_valid += 1;
    }

    return count_valid;
}

fn part2(kv_pairs: &[String]) -> usize {
    let mut fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]; // , "cid"
    fields.sort();

    let mut count_valid = 0;

    let mut passport = Vec::new();
    for pair in kv_pairs {
        if pair.is_empty() {
            // check if complete and increment count
            passport.sort();
            if fields == passport[..] {
                count_valid += 1;
            }
            passport.clear();
        } else {
            let mut split = pair.split(":");
            let key = split.next().unwrap();
            let value = split.next().unwrap();

            if key == "cid" || value_is_valid(key, value).is_err() {
                continue;
            }

            passport.push(key);
        }
    }

    // also check the last passport (if there is no newline after it)
    passport.sort();
    if fields == passport[..] {
        count_valid += 1;
    }

    return count_valid;
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
// hgt (Height) - a number followed by either cm or in:
//     If cm, the number must be at least 150 and at most 193.
//     If in, the number must be at least 59 and at most 76.
// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
// pid (Passport ID) - a nine-digit number, including leading zeroes.
// cid (Country ID) - ignored, missing or not.
fn value_is_valid(key: &str, value: &str) -> Result<(), ()> {
    match key {
        "byr" => {
            let value = value.parse::<usize>().map_err(|_| ())?;
            if 1920 <= value && value <= 2002 {
                return Ok(());
            }
        }
        "iyr" => {
            let value = value.parse::<usize>().map_err(|_| ())?;
            if 2010 <= value && value <= 2020 {
                return Ok(());
            }
        }
        "eyr" => {
            let value = value.parse::<usize>().map_err(|_| ())?;
            if 2020 <= value && value <= 2030 {
                return Ok(());
            }
        }
        "hgt" => {
            if value.len() < 3 {
                return Err(());
            }
            let unit = &value[value.len() - 2..];
            let value = value[..value.len() - 2].parse::<usize>().map_err(|_| ())?;

            match unit {
                "cm" => {
                    if 150 <= value && value <= 193 {
                        return Ok(());
                    }
                }
                "in" => {
                    if 59 <= value && value <= 76 {
                        return Ok(());
                    }
                }
                _ => return Err(()),
            }
        }
        "hcl" => {
            if value.chars().nth(0).unwrap() != '#' || value.len() != 7 {
                return Err(());
            }
            if value[1..]
                .chars()
                .all(|c| ('0' <= c && c <= '9') || ('a' <= c && c <= 'f'))
            {
                return Ok(());
            }
        }
        "ecl" => match value {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {
                return Ok(());
            }
            _ => return Err(()),
        },
        "pid" => {
            if value.len() != 9 {
                return Err(());
            }
            return value.parse::<usize>().map(|_| ()).map_err(|_| ());
        }
        "cid" => return Ok(()),
        _ => unreachable!(),
    }
    return Err(());
}
