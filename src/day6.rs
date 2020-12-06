use crate::utils::read_lines;
use itertools::Itertools;

pub fn day6() {
    let lines: Vec<String> = read_lines("day6.input").collect();
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(lines: &[String]) -> usize {
    let mut yes: [bool; 26] = Default::default();
    let mut yes_per_group = Vec::new();

    for line in lines {
        if line.is_empty() {
            let count = yes.iter().filter(|y| **y).count();
            yes_per_group.push(count);
            yes = Default::default();
        } else {
            for c in line.chars() {
                yes[c as usize - 'a' as usize] = true;
            }
        }
    }

    // also count the last group if there is no empty line after it
    let count = yes.iter().filter(|y| **y).count();
    yes_per_group.push(count);

    return yes_per_group.iter().sum();
}

fn part2(lines: &[String]) -> usize {
    let mut yes: [bool; 26] = [true; 26];
    let mut yes_per_group = Vec::new();

    for line in lines {
        if line.is_empty() {
            let count = yes.iter().filter(|y| **y).count();
            yes_per_group.push(count);
            yes = [true; 26];
        } else {
            for c in 'a'..='z' {
                if !line.contains(c) {
                    yes[c as usize - 'a' as usize] = false;
                }
            }
        }
    }

    // also count the last group if there is no empty line after it
    let count = yes.iter().filter(|y| **y).count();
    yes_per_group.push(count);

    return yes_per_group.iter().sum();
}
