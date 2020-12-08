use crate::utils::read_lines;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn day7() {
    let lines = read_lines("day7.input").collect::<Vec<String>>();
    let rules = lines
        .iter()
        .map(|line| parse_line(&line))
        .collect::<Vec<_>>();
    println!("Part 1: {}", part1(&rules));
    println!("Part 2: {}", part2(&rules));
}

fn part1(rules: &[(&str, Vec<(usize, &str)>)]) -> usize {
    // let rules = lines.iter().map(|line| parse_line(line)).collect::<Vec<_>>();

    let mut change = true;
    let mut queries = HashSet::new();
    queries.insert("shiny gold");
    while change {
        change = false;
        for (outer_color, contents) in rules.iter() {
            if !queries.contains(outer_color) {
                for (_, inner_color) in contents {
                    if queries.contains(inner_color) {
                        change = true;
                        queries.insert(outer_color);
                    }
                }
            }
        }
    }

    // dbg!(&queries);

    queries.len() - 1
}

// light red bags contain 1 bright white bag, 2 muted yellow bags.
fn parse_line(line: &str) -> (&str, Vec<(usize, &str)>) {
    let mut iter = line[..line.len() - 1].split(" bags contain ");
    let color = iter.next().unwrap();
    let contents = iter.next().unwrap();

    let contents = if contents == "no other bags" {
        Vec::new()
    } else {
        contents
            .split(", ")
            .map(|bag| {
                let mut iter = bag.splitn(2, " ");
                let count = iter.next().unwrap().parse::<usize>().unwrap();
                let color = iter.next().unwrap();
                let color = if count == 1 {
                    &color[..color.len() - 4]
                } else {
                    &color[..color.len() - 5]
                };
                (count, color)
            })
            .collect()
    };

    (color, contents)
}

fn part2(rules: &[(&str, Vec<(usize, &str)>)]) -> usize {
    let rules: HashMap<_, _> = rules.iter().cloned().collect();

    // dbg!(&rules);

    count_bags("shiny gold", &rules) - 1
}

fn count_bags(bag: &str, rules: &HashMap<&str, Vec<(usize, &str)>>) -> usize {
    // dbg!(bag);
    rules
        .get(bag)
        .unwrap()
        .iter()
        .map(|(count, inner_bag)| count * count_bags(inner_bag, rules))
        .sum::<usize>()
        + 1
}
