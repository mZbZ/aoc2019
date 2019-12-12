extern crate elapsed;
use elapsed::measure_time;
use std::collections::HashMap;

const WHITE: isize = 1;
const BLACK: isize = 0;

fn main() {
    println!("AOC 2019 Day 11");

    let (_, input) = measure_time(|| parse_intcode(include_str!("../input")));
    // println!("Parse time = {}", parse_time);

    let (solve_time, output) = measure_time(|| part_both(input.clone(), BLACK));
    println!("Part 1 answer = {:?}, Solve time = {}", output, solve_time);

    let (solve_time, output) = measure_time(|| part_both(input.clone(), WHITE));
    println!("Part 2 answer = {:?}, Solve time = {}", output, solve_time);
}

fn part_both(mut prog: Vec<isize>, start_col: isize) -> usize {
    let mut hull: HashMap<(isize, isize), isize> = HashMap::new();
    let mut direction = (0, 1);
    let mut idx = 0;
    let mut position = (0, 0);
    let mut rel_base = 0;
    hull.insert(position, start_col);
    loop {
        let hull_colour = match hull.get(&position) {
            Some(col) => *col,
            None => BLACK,
        };
        let (status, result) = intcode(&mut prog, &mut vec![hull_colour], &mut idx, &mut rel_base);

        if result.len() != 2 {
            panic!("This shoudln't happen");
        }
        hull.insert(position, result[0]);
        match result[1] {
            0 => direction = (-direction.1, direction.0),
            1 => direction = (direction.1, -direction.0),
            _ => panic!("Rotate where now?"),
        }
        position = (position.0 + direction.0, position.1 + direction.1);

        if status == Output::Halt {
            break;
        }
    }
    print_hull(&hull);
    hull.len()
}

fn print_hull(hull: &HashMap<(isize, isize), isize>) {
    let min_x = hull.keys().min_by(|(x, _), (x1, _)| x.cmp(x1)).unwrap().0;
    let max_x = hull.keys().max_by(|(x, _), (x1, _)| x.cmp(x1)).unwrap().0;
    let min_y = hull.keys().min_by(|(_, y), (_, y1)| y.cmp(y1)).unwrap().1;
    let max_y = hull.keys().max_by(|(_, y), (_, y1)| y.cmp(y1)).unwrap().1;
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            match hull.get(&(x, y)) {
                Some(&WHITE) => print!("#"),
                Some(&BLACK) => print!(" "),
                None => print!(" "),
                _ => panic!("Should never happen!"),
            }
        }
        println!();
    }
}

fn parse_intcode(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

#[derive(Debug, PartialEq)]
enum Output {
    Halt,
    Continue,
}

fn parse_instruction(instruc: isize) -> (usize, usize, usize, usize) {
    (
        (instruc % 100) as usize,
        (instruc / 100 % 10) as usize,
        (instruc / 1000 % 10) as usize,
        (instruc / 10_000 % 10) as usize,
    )
}

fn check_size(prog: &mut Vec<isize>, index: usize) {
    if index >= prog.len() {
        prog.resize_with(index + 1, Default::default);
    }
}

fn mode_to_idx(
    prog: &mut Vec<isize>,
    mode: usize,
    idx: usize,
    rel_base: isize,
    add: usize,
) -> usize {
    match mode {
        0 => prog[idx + add] as usize,
        1 => idx + add,
        2 => (rel_base + prog[idx + add]) as usize,
        _ => panic!("Should never happen"),
    }
}

fn intcode(
    prog: &mut Vec<isize>,
    input: &mut Vec<isize>,
    idx: &mut usize,
    rel_base: &mut isize,
) -> (Output, Vec<isize>) {
    let mut output = (Output::Continue, vec![]);
    while *idx < prog.len() {
        if prog[*idx] % 100 == 99 {
            output.0 = Output::Halt;
            return output;
        } else {
            let (op, f_mode, s_mode, t_mode) = parse_instruction(prog[*idx]);
            check_size(prog, *idx + 1);
            check_size(prog, *idx + 2);
            check_size(prog, *idx + 3);
            let f_idx = mode_to_idx(prog, f_mode, *idx, *rel_base, 1);
            let s_idx = mode_to_idx(prog, s_mode, *idx, *rel_base, 2);
            let t_idx = mode_to_idx(prog, t_mode, *idx, *rel_base, 3);

            match op {
                1 => {
                    check_size(prog, f_idx);
                    check_size(prog, s_idx);
                    check_size(prog, t_idx);

                    prog[t_idx] = prog[f_idx] + prog[s_idx];
                    *idx += 4;
                }
                2 => {
                    check_size(prog, f_idx);
                    check_size(prog, s_idx);
                    check_size(prog, t_idx);
                    prog[t_idx] = prog[f_idx] * prog[s_idx];
                    *idx += 4;
                }
                3 => {
                    check_size(prog, f_idx);

                    if let Some(input_val) = input.pop() {
                        prog[f_idx] = input_val;
                        *idx += 2;
                    } else {
                        return output;
                    }
                }
                4 => {
                    check_size(prog, f_idx);

                    *idx += 2;
                    output.1.push(prog[f_idx]);
                }
                5 => {
                    check_size(prog, f_idx);
                    check_size(prog, s_idx);

                    if prog[f_idx] != 0 {
                        *idx = prog[s_idx] as usize;
                    } else {
                        *idx += 3;
                    }
                }
                6 => {
                    check_size(prog, f_idx);
                    check_size(prog, s_idx);

                    if prog[f_idx] == 0 {
                        *idx = prog[s_idx] as usize;
                    } else {
                        *idx += 3;
                    }
                }
                7 => {
                    check_size(prog, f_idx);
                    check_size(prog, s_idx);
                    check_size(prog, t_idx);
                    if prog[f_idx] < prog[s_idx] {
                        prog[t_idx] = 1;
                    } else {
                        prog[t_idx] = 0;
                    }
                    *idx += 4;
                }
                8 => {
                    check_size(prog, f_idx);
                    check_size(prog, s_idx);
                    check_size(prog, t_idx);
                    if prog[f_idx] == prog[s_idx] {
                        prog[t_idx] = 1;
                    } else {
                        prog[t_idx] = 0;
                    }
                    *idx += 4;
                }
                9 => {
                    check_size(prog, f_idx);
                    *rel_base += prog[f_idx];
                    *idx += 2;
                }
                _ => panic!("Should never happen"),
            }
            check_size(prog, *idx);
        }
    }
    output
}
