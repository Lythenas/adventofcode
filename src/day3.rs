use crate::utils::read_lines;

pub fn day3() {
     let lines : Vec<_> = read_lines("day3.input").map(|line| line.as_bytes().to_vec()).collect();
     println!("Part 1: {}", part1(&lines));
     println!("Part 2: {}", part2(&lines));
}

fn part1(lines: &[Vec<u8>]) -> usize {
    let slope_x = 3;
    let slope_y = 1;

    return check_slope(lines, slope_x, slope_y);
}

fn part2(lines: &[Vec<u8>]) -> usize {
     return [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter()
         .map(|(slope_x, slope_y)| check_slope(lines, *slope_x, *slope_y))
         .fold(1, |a, b| a*b);
}

fn check_slope(lines: &[Vec<u8>], slope_x: usize, slope_y: usize) -> usize {
    let width = lines[0].len();
    let height = lines.len();
    // println!("width {}, height {}", width, height);

    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;

    while y < height - 1 {
        x += slope_x;
        y += slope_y;

        // println!("{:?}", (x, y));
        if lines[y][x % width] == '#' as u8 {
            trees += 1;
        }
    }

    return trees;
}
