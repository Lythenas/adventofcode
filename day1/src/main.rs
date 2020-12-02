use std::io::{self, BufRead};

fn main() {
    let lines : Vec<_> = read_lines().map(|line| line.parse::<usize>().unwrap()).collect();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(lines: &[usize]) -> usize {
    for line1 in lines.iter() {
        for line2 in lines.iter() {
            if line1 + line2 == 2020 {
                return line1 * line2;
            }
        }
    }
    unreachable!();
}

fn part2(lines: &[usize]) -> usize {
    for line1 in lines.iter() {
        for line2 in lines.iter() {
            for line3 in lines.iter() {
                if line1 + line2 + line3 == 2020 {
                    return line1 * line2 * line3;
                }
            }
        }
    }
    unreachable!();
}

fn read_lines() -> impl Iterator<Item=String> {
    io::BufReader::new(io::stdin()).lines().map(|line| line.unwrap())
}
