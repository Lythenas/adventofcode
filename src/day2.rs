use crate::utils::read_lines;

pub fn day2() {
    let lines : Vec<_> = read_lines("day2.input").collect();
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(lines: &[String]) -> usize {
    return lines.iter().filter_map(|line| {
        let parts : Vec<_> = line.split(":").map(String::from).collect();
        if let [policy, password] = &parts[..] {
            let (times, letter) = {
                let mut iter = policy.split(" ");
                (iter.next().unwrap().to_owned(), iter.next().unwrap().chars().next().unwrap())
            };
            let (policy_min, policy_max): (usize, usize) = {
                let mut iter = times.split("-");
                (iter.next().unwrap().to_owned().parse().unwrap(), iter.next().unwrap().to_owned().parse().unwrap())
            };

            // check if valid
            let letter_count = password.chars().filter(|c| *c == letter).count();
            if policy_min <= letter_count && letter_count <= policy_max {
                return Some(());
            } else {
                return None;
            }
        } else {
            unreachable!();
        }
    }).count();
}

fn part2(lines: &[String]) -> usize {
    return lines.iter().filter_map(|line| {
        let parts : Vec<_> = line.split(":").map(String::from).collect();
        if let [policy, password] = &parts[..] {
            let (times, letter) = {
                let mut iter = policy.split(" ");
                (iter.next().unwrap().to_owned(), iter.next().unwrap().chars().next().unwrap())
            };
            let (policy_loc1, policy_loc2): (usize, usize) = {
                let mut iter = times.split("-");
                (iter.next().unwrap().to_owned().parse().unwrap(), iter.next().unwrap().to_owned().parse().unwrap())
            };

            // check if valid
            let first_is_letter = password.as_bytes()[policy_loc1] == letter as u8;
            let second_is_letter = password.as_bytes()[policy_loc2] == letter as u8;
            if (first_is_letter && !second_is_letter) || (!first_is_letter && second_is_letter) {
                return Some(());
            } else {
                return None;
            }
        } else {
            unreachable!();
        }
    }).count();
}
