const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LENGTH: usize = WIDTH * HEIGHT;

const WHITE: char = '#';
const EMPTY: char = '.';
const BLACK: char = ' ';

fn main() {
    println!("AOC 2019 Day8");

    let input = include_str!("../input");

    println!("Part 1: {:?}", part1(parse_layers(input)));
    println!("Part 2: {:?}", part2(parse_layers(input)));
}

fn parse_layers(input: &str) -> Vec<&str> {
    let mut range_max = LENGTH;
    let mut range_min = 0;
    let mut parsed = vec![];
    while let Some(layer) = input.get(range_min..range_max) {
        parsed.push(layer);
        range_max += LENGTH;
        range_min += LENGTH;
    }
    parsed
}

fn part1(input: Vec<&str>) -> usize {
    let (_, result) = input
        .iter()
        .fold((usize::max_value(), 0), |(mut min, mut res), x| {
            let mut zero_count = 0;
            let mut one_count = 0;
            let mut two_count = 0;

            x.chars().for_each(|y| match y {
                '0' => zero_count += 1,
                '1' => one_count += 1,
                '2' => two_count += 1,
                _ => panic!("Should never happen"),
            });
            if zero_count < min {
                min = zero_count;
                res = one_count * two_count;
            }
            (min, res)
        });
    result
}

fn part2(input: Vec<&str>) {
    let mut final_layer = [[EMPTY; WIDTH]; HEIGHT];
    input.iter().for_each(|layer| {
        layer.chars().enumerate().for_each(|(p_idx, code)| {
            let pix = match code {
                '2' => EMPTY,
                '1' => WHITE,
                '0' => BLACK,
                _ => panic!("Should never happen"),
            };
            if pix != EMPTY && final_layer[p_idx / WIDTH][p_idx % WIDTH] == EMPTY {
                final_layer[p_idx / WIDTH][p_idx % WIDTH] = pix;
            }
        })
    });
    for l in final_layer.iter() {
        for p in l.iter() {
            print!("{}", p);
        }
        println!();
    }
}
