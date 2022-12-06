use std::fs;

fn main() {
    first();
    second();
}

fn second() {
    let contents = fs::read_to_string("/Users/delikristof.demeter/Projects/adventOfCode2022/sources/4/input.txt")
        .expect("Should be able to open file");
    let mut sum = 0;
    for a in contents.split("\n") {
        let b = a.split(",")
            .flat_map(|part| part.split("-"))
            .map(|ch| ch.parse().unwrap())
            .collect::<Vec<i32>>();
        if (b[0] >= b[2] && b[0] <= b[3]) || (b[1] >= b[2] && b[1] <= b[3])
            || (b[2] >= b[0] && b[2] <= b[1]) || (b[3] >= b[0] && b[3] <= b[1]) { sum += 1 };
    }
    println!("Second task: {}", sum);
}

fn first() {
    let contents = fs::read_to_string("/Users/delikristof.demeter/Projects/adventOfCode2022/sources/4/input.txt")
        .expect("Should be able to open file");
    let mut sum = 0;
    for a in contents.split("\n") {
        let b = a.split(",")
            .flat_map(|part| part.split("-"))
            .map(|ch| ch.parse().unwrap())
            .collect::<Vec<i32>>();
        if (b[0] <= b[2] && b[1] >= b[3]) || (b[0] >= b[2] && b[1] <= b[3]) { sum += 1 };
    }
    println!("First task: {}", sum);
}