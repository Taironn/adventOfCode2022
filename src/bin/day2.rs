use std::collections::HashMap;
use std::fs;

fn main() {
    first();
    second();
}

fn  second() {
    let contents = fs::read_to_string("/Users/delikristof.demeter/Projects/adventOfCode2022/sources/2/input.txt")
        .expect("Should be able to open file");
    let scores = HashMap::from([
        ("A", 1),
        ("B", 2),
        ("C", 3)
    ]);

    let mut sum = 0;
    for a in contents.split("\n") {
        let b = a.split(" ").collect::<Vec<&str>>();
        let mut result = 0;
        let enemy = b[0];
        result += match b[1]  {
            "X" => if enemy.eq("A") {3} else if enemy.eq("B") { 1 } else { 2 },
            "Y" => scores.get(b[0]).expect("notnull") + 3,
            "Z" => if enemy.eq("A") {8} else if enemy.eq("B") { 9 } else { 7 },
            default => panic!("Invalid input")
        };
        sum += result;
    }

    println!("Total score: {sum}")

}

fn first() {
    let contents = fs::read_to_string("/Users/delikristof.demeter/Projects/adventOfCode2022/sources/2/input.txt")
        .expect("Should be able to open file");
    let scores = HashMap::from([
        ("A", 1),
        ("B", 2),
        ("C", 3),
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);

    let mut sum = 0;
    for a in contents.split("\n") {
        let b = a.split(" ").collect::<Vec<&str>>();
        let mut result = 0;
        result += scores.get(b[1]).expect("Should not be null!");
        result += game(b[0], b[1]);
        sum += result;
    }

    println!("Total score: {sum}")
}

fn game(en: &str, pl: &str) -> i32 {
    match en {
        "A" => if pl.eq("X") {3} else if pl.eq("Y") { 6 } else { 0 },
        "B" => if pl.eq("Y") {3} else if pl.eq("Z") { 6 } else { 0 },
        "C" => if pl.eq("Z") {3} else if pl.eq("X") { 6 } else { 0 },
        default => panic!("Invalid input")
    }
}