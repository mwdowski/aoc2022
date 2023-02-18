mod days;

fn main() {
    assert_eq!(days::day1::part1(), Some(69912));
    assert_eq!(days::day1::part2(), 208180);
    assert_eq!(days::day2::part1(), 8392);
    assert_eq!(days::day2::part2(), 10116);

    println!("OK");
}
