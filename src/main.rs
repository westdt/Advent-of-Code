mod day_01_1;
mod day_01_2;
mod day_02_1;
mod day_02_2;
mod day_03_1;
mod day_03_2;
mod day_04_1;
mod day_04_2;
mod day_05_1;
mod day_05_2;
mod day_06_1;
mod day_06_2;
mod day_07_1;
mod day_07_2;
mod day_08_1;
mod day_08_2;
mod day_09_1;
mod day_09_2;

fn main() {
    println!(
        "Day 1, part 1: {}",
        day_01_1::solve(include_str!("day_01.txt"))
    );

    println!(
        "Day 1, part 2: {}",
        day_01_2::solve(include_str!("day_01.txt"))
    );

    println!(
        "Day 2, part 1: {}",
        day_02_1::solve(include_str!("day_02.txt"))
    );

    println!(
        "Day 2, part 2: {}",
        day_02_2::solve(include_str!("day_02.txt"))
    );

    println!(
        "Day 3, part 1: {}",
        day_03_1::solve(include_str!("day_03.txt"))
    );

    println!(
        "Day 3, part 2: {}",
        day_03_2::solve(include_str!("day_03.txt"))
    );

    println!(
        "Day 4, part 1: {}",
        day_04_1::solve(include_str!("day_04.txt"))
    );

    println!(
        "Day 4, part 2: {}",
        day_04_2::solve(include_str!("day_04.txt"))
    );

    println!(
        "Day 5, part 1: {}",
        day_05_1::solve(include_str!("day_05.txt"))
    );

    println!(
        "Day 5, part 2: {}",
        day_05_2::solve(include_str!("day_05.txt"))
    );

    println!(
        "Day 6, part 1: {}",
        day_06_1::solve(include_str!("day_06.txt"))
    );

    println!(
        "Day 6, part 2: {}",
        day_06_2::solve(include_str!("day_06.txt"))
    );

    println!(
        "Day 7, part 1: {}",
        day_07_1::solve(include_str!("day_07.txt"))
    );

    println!(
        "Day 7, part 2: {}",
        day_07_2::solve(include_str!("day_07.txt"))
    );

    println!(
        "Day 8, part 1: {}",
        day_08_1::solve(include_str!("day_08.txt"))
    );

    println!(
        "Day 8, part 2: {}",
        day_08_2::solve(include_str!("day_08.txt"))
    );

    println!(
        "Day 9, part 1: {}",
        day_09_1::solve(include_str!("day_09.txt"))
    );

    println!(
        "Day 9, part 2: {}",
        day_09_2::solve(include_str!("day_09.txt"))
    );
}
