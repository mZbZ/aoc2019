fn main() {
    let mut num = 264_360;
    let mut p1_count = 0;
    let mut p2_count = 0;
    while num < 746_325 {
        if check_increasing(num) {
            if check_consecutive(num) {
                p1_count += 1;
            }
            if check_consecutive_no_grp(num) {
                p2_count += 1;
            }
        }
        num += 1;
    }

    println!("part 1 {}", p1_count);
    println!("part 2 {}", p2_count);
}

fn check_consecutive(num: usize) -> bool {
    num / 100_000 % 10 == num / 10_000 % 10
        || num / 10_000 % 10 == num / 1_000 % 10
        || num / 1_000 % 10 == num / 100 % 10
        || num / 100 % 10 == num / 10 % 10
        || num / 10 % 10 == num % 10
}

fn check_consecutive_no_grp(num: usize) -> bool {
    num / 100_000 % 10 == num / 10_000 % 10 && num / 10_000 % 10 != num / 1_000 % 10
        || num / 10_000 % 10 == num / 1_000 % 10
            && num / 10_000 % 10 != num / 100_000 % 10
            && num / 1_000 % 10 != num / 100 % 10
        || num / 1_000 % 10 == num / 100 % 10
            && num / 10_000 % 10 != num / 1_000 % 10
            && num / 100 % 10 != num / 10 % 10
        || num / 100 % 10 == num / 10 % 10
            && num / 1_000 % 10 != num / 100 % 10
            && num / 10 % 10 != num % 10
        || num / 10 % 10 == num % 10 && num / 100 % 10 != num / 10 % 10
}

fn check_increasing(num: usize) -> bool {
    num / 100_000 % 10 <= num / 10_000 % 10
        && num / 10_000 % 10 <= num / 1_000 % 10
        && num / 1_000 % 10 <= num / 100 % 10
        && num / 100 % 10 <= num / 10 % 10
        && num / 10 % 10 <= num % 10
}
