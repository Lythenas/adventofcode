use crate::utils::read_lines;
use std::collections::{HashMap, HashSet};

pub fn day10() {
    let numbers = read_lines("day10.input")
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));
}

fn part1(numbers: &[usize]) -> usize {
    let mut numbers = numbers.to_owned();
    numbers.sort();

    let max = numbers.iter().max().unwrap();
    let internal_adapter = max + 3;

    numbers.push(internal_adapter);

    let mut num_1_jolt_diff = 0;
    let mut num_3_jolt_diff = 0;

    let mut current_jolts = 0;

    for num in numbers {
        let difference = num - current_jolts;
        assert!(1 <= difference && difference <= 3);

        match difference {
            1 => num_1_jolt_diff += 1,
            3 => num_3_jolt_diff += 1,
            _ => {},
        }

        current_jolts = num;
    }

    num_1_jolt_diff * num_3_jolt_diff
}

fn part2(numbers: &[usize]) -> usize {
    let mut numbers = numbers.to_owned();
    numbers.sort();

    let max = numbers.iter().max().unwrap();
    let internal_adapter = max + 3;
    numbers.push(internal_adapter);

    // solve using dynamic programming

    let mut counted_paths = HashMap::<usize, usize>::new();
    let mut number_set = HashSet::<usize>::new();

    for &num in &numbers {
        number_set.insert(num);
    }

    count_paths(&mut counted_paths, &number_set, 0)
}

fn count_paths(counted_paths: &mut HashMap<usize, usize>, numbers: &HashSet<usize>, start: usize) -> usize {
    if counted_paths.contains_key(&start) {
        return *counted_paths.get(&start).unwrap();
    }

    let mut res = 0;
    let mut found_candidate = false;

    for i in 1..=3 {
        let candidate = start + i;
        if numbers.contains(&candidate) {
            res += count_paths(counted_paths, numbers, candidate);
            found_candidate = true;
        }

        counted_paths.insert(start, res);
    }

    if !found_candidate {
        res += 1;
    }

    res
}
