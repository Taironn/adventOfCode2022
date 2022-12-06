use std::collections::{HashSet};
use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    first();
    second();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn  second() {
    let contents = fs::read_to_string("/Users/delikristof.demeter/Projects/adventOfCode2022/sources/6/input.txt")
        .expect("Should be able to open file");
    let  input:Vec<char> = contents.chars().collect();
    let  mut first_match = 0;
    for i in 14..contents.len() {
        if all_distinct(&input[i-14..i]) {
            first_match = i;
            break;
        }
    }
    println!("Second task: {}", first_match);
}

fn all_distinct(part: &[char]) -> bool {
    let set: HashSet<&char> = part.iter().collect();
    return set.len() == part.len()
}

fn first() {
    let contents = fs::read_to_string("/Users/delikristof.demeter/Projects/adventOfCode2022/sources/6/input.txt")
        .expect("Should be able to open file");
    let  input:Vec<char> = contents.chars().collect();
    let  mut first_match = 0;
    for i in 4..contents.len() {
        if all_distinct_4(&input[i-4..i]) {
            first_match = i;
            break;
        }
    }
    println!("First task: {}", first_match);
}

fn all_distinct_4(part: &[char]) -> bool {
    !(part[0] == part[1] || part[0] == part[2] || part[0] == part[3]
        || part[1] == part[2] || part[1] == part[3]
        || part[2] == part[3])
}