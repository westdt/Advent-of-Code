mod day_01_1;
mod day_01_2;
mod day_02_1;

fn main() {
    println!(
        "Day 1, part 1: {}",
        day_01_1::solve(include_str!("day_01_1.txt"))
    );

    println!(
        "Day 1, part 2: {}",
        day_01_2::solve(include_str!("day_01_2.txt"))
    );

    println!(
        "Day 2, part 1: {}",
        day_02_1::solve(include_str!("day_02_1.txt"))
    );
}
