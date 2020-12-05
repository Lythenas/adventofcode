use crate::utils::read_lines;
use itertools::Itertools;

pub fn day5() {
    let lines: Vec<String> = read_lines("day5.input").collect();
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

// F: front
// B: back
// L: left
// R: right
//
// 128 rows
// 8 columns

fn parse_line(line: &str) -> usize {
    let row_str = line[0..7]
        .chars()
        .map(|c| match c {
            'F' => '0',
            'B' => '1',
            _ => unreachable!(),
        })
        .collect::<String>();
    let col_str = line[7..]
        .chars()
        .map(|c| match c {
            'L' => '0',
            'R' => '1',
            _ => unreachable!(),
        })
        .collect::<String>();

    let row = usize::from_str_radix(&row_str, 2).unwrap();
    let col = usize::from_str_radix(&col_str, 2).unwrap();

    row * 8 + col
}

fn part1(lines: &[String]) -> usize {
    lines.iter().map(|line| parse_line(&*line)).max().unwrap()
}

fn part2(lines: &[String]) -> usize {
    let seats = lines
        .iter()
        .map(|line| parse_line(&*line))
        .sorted()
        .collect::<Vec<_>>();

    let mut offset = 0;

    for (i, s) in seats.iter().enumerate().skip(1) {
        if i + offset != *s {
            if seats[i - 1] == *s - 2 {
                println!("{}, {}", seats[i - 1], s);
                return *s - 1;
            }
        } else {
            offset = *s - i;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("BFFFBBFRRR"), 567);
        assert_eq!(parse_line("FFFBBBFRRR"), 119);
        assert_eq!(parse_line("BBFFBBFRLL"), 820);
    }
}
