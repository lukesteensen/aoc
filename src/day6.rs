use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let times = lines
        .next()
        .expect("times")
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().expect("parse time"));
    let distances = lines
        .next()
        .expect("distances")
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().expect("parse distance"));

    times
        .zip(distances)
        .map(|(time, distance)| {
            (0..time)
                .filter_map(|t| {
                    let speed = t;
                    let d = speed * (time - t);
                    (d > distance).then(|| ())
                })
                .count()
        })
        .reduce(|x, y| x * y)
        .expect("reduce")
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let time = lines
        .next()
        .expect("times")
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .expect("time");
    let distance = lines
        .next()
        .expect("distances")
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .expect("distance");

    (0..time)
        .filter_map(|t| {
            let speed = t;
            let d = speed * (time - t);
            (d > distance).then(|| ())
        })
        .count()
}
