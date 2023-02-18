use std::fs::read_to_string;

pub fn part1() -> Option<i32> {
    let file_contents = read_to_string("./inputs/in01").expect("Can't open file");

    file_contents
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|item| item.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
}

pub fn part2() -> i32 {
    let file_contents = read_to_string("./inputs/in01").expect("Can't open file");

    let mut sums = file_contents
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|item| item.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    sums.sort_by(|a, b| b.cmp(a));

    sums.iter().take(3).sum()
}
