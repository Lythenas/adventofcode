use crate::utils::read_lines;

pub fn day9() {
    let numbers = read_lines("day9.input")
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let target = part1(&numbers);
    println!("Part 1: {}", target);
    println!("Part 2: {}", part2(&numbers, target));
}

const PREAMBLE_LEN: usize = 25;

fn part1(numbers: &[usize]) -> usize {
    for i in PREAMBLE_LEN..numbers.len() {
        let num = numbers[i];

        if !find_sum(&numbers[i - PREAMBLE_LEN..i], num) {
            return num;
        }
    }

    unreachable!();
}

fn find_sum(preamble: &[usize], target: usize) -> bool {
    for preamble_i in 0..preamble.len() {
        for preamble_j in 0..preamble.len() {
            if preamble_i == preamble_j {
                continue;
            }

            if target == preamble[preamble_i] + preamble[preamble_j] {
                return true;
            }
        }
    }

    return false;
}

fn part2(numbers: &[usize], target: usize) -> usize {
    for i in 0..numbers.len() {
        if let Ok((smallest, largest)) = find_continuous_sum(&numbers[i..], target) {
            return smallest + largest;
        }
    }

    unreachable!();
}

fn find_continuous_sum(numbers: &[usize], target: usize) -> Result<(usize, usize), ()> {
    let mut sum = 0;
    let mut smallest = usize::MAX;
    let mut largest = 0;
    for i in 0..numbers.len() {
        let num = numbers[i];
        sum += num;

        if num < smallest {
            smallest = num;
        }
        if num > largest {
            largest = num;
        }

        if sum == target {
            return Ok((smallest, largest));
        } else if sum > target {
            return Err(());
        }
    }

    Err(())
}
