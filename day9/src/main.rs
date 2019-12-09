extern crate elapsed;
use elapsed::measure_time;

fn main() {
    println!("AOC 2019 Day9");

    let input = parse_intcode(include_str!("../input"));

    let _t_str0 = parse_intcode("1102,34915192,34915192,7,4,7,99,0");
    assert_eq!(
        part_both(_t_str0, vec![1]).pop().unwrap().to_string().len(),
        16
    );
    let _t_str1 = parse_intcode("104,1125899906842624,99");
    assert_eq!(
        part_both(_t_str1, vec![1]).pop().unwrap(),
        1_125_899_906_842_624
    );
    let _t_str2 = parse_intcode("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    println!("Test Quine Output {:?}", part_both(_t_str2, vec![]));

    let (solve_time, output) = measure_time(|| part_both(input.clone(), vec![1]));
    println!("Part 1 answer = {:?}, Solve time = {}", output, solve_time);

    let (solve_time, output) = measure_time(|| part_both(input.clone(), vec![2]));
    println!("Part 2 answer = {:?}, Solve time = {}", output, solve_time);
}

fn part_both(mut prog: Vec<isize>, mut input: Vec<isize>) -> Vec<isize> {
    intcode(&mut prog, &mut input, &mut 0).1
}

fn parse_intcode(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

#[derive(Debug)]
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

fn intcode(prog: &mut Vec<isize>, input: &mut Vec<isize>, idx: &mut usize) -> (Output, Vec<isize>) {
    let mut output = (Output::Continue, vec![]);
    let mut rel_base = 0isize;
    while *idx < prog.len() {
        if prog[*idx] % 100 == 99 {
            output.0 = Output::Halt;
            return output;
        } else {
            let (op, f_mode, s_mode, t_mode) = parse_instruction(prog[*idx]);
            check_size(prog, *idx + 1);
            check_size(prog, *idx + 2);
            check_size(prog, *idx + 3);
            let f_idx = mode_to_idx(prog, f_mode, *idx, rel_base, 1);
            let s_idx = mode_to_idx(prog, s_mode, *idx, rel_base, 2);
            let t_idx = mode_to_idx(prog, t_mode, *idx, rel_base, 3);
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
                    prog[f_idx] = input.pop().expect("Not enough input for program!");
                    *idx += 2
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
                    rel_base += prog[f_idx];
                    *idx += 2;
                }
                _ => panic!("Should never happen"),
            }
            check_size(prog, *idx);
        }
    }
    output
}
