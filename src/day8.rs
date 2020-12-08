use crate::utils::read_lines;

pub fn day8() {
    let lines = read_lines("day8.input").collect::<Vec<String>>();
    let instrs = lines.iter().map(|line| {
        let mut iter = line.split(' ');
        let instr = iter.next().unwrap();
        let arg = iter.next().unwrap().parse::<isize>().unwrap();
        (instr, arg)
    }).collect::<Vec<_>>();
    // dbg!(instrs);
    println!("Part 1: {}", part1(&instrs));
    println!("Part 2: {}", part2(&instrs));
}

fn part1(instrs: &[(&str, isize)]) -> isize {
    let mut instrs = instrs.to_owned();
    let mut pc = 0isize;

    let mut acc = 0isize;

    loop {
        let old_pc = pc;
        let (instr, arg) = instrs[pc as usize];
        match instr {
            "_visited" => return acc,
            "nop" => {},
            "acc" => acc += arg,
            "jmp" => pc += arg - 1,
            _ => unimplemented!(),
        }

        pc += 1;
        instrs[old_pc as usize] = ("_visited", 0);
    }
}

fn part2(instrs: &[(&str, isize)]) -> isize {
    for (i, (instr, arg)) in instrs.iter().enumerate() {
        if *instr == "jmp" {
            let mut instrs = instrs.to_owned();
            instrs[i] = ("nop", *arg);
            if let Ok(acc) = try_execute(&instrs) {
                return acc;
            }
        } else if *instr == "nop" {
            let mut instrs = instrs.to_owned();
            instrs[i] = ("jmp", *arg);
            if let Ok(acc) = try_execute(&instrs) {
                return acc;
            }
        }
    }

    unreachable!()
}

fn try_execute(instrs: &[(&str, isize)]) -> Result<isize, ()> {
    let mut instrs : Vec<_> = instrs.into();
    let mut pc = 0isize;

    let mut acc = 0isize;

    while (pc as usize) < instrs.len() {
        let old_pc = pc;
        let (instr, arg) = instrs[pc as usize];
        match instr {
            "_visited" => return Err(()),
            "nop" => {},
            "acc" => acc += arg,
            "jmp" => pc += arg - 1,
            _ => unimplemented!(),
        }

        pc += 1;
        instrs[old_pc as usize] = ("_visited", 0);
    }

    return Ok(acc);
}

