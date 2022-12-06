use std::fs;

fn  main() {
    first();
    second();
}

fn second() {
    let contents = fs::read_to_string("/Users/delikristof.demeter/Projects/adventOfCode2022/sources/1/input.txt")
        .expect("Should be able to open file");
    let mut sum = 0;
    let mut sums = Vec::new();
    for a in contents.split("\n") {
        if a.eq("") {
            sums.push(sum);
            sum = 0;
        } else {
            sum += a.parse::<i32>().unwrap();
        }
    }
    sums.sort();
    sums.reverse();
    let  top_3_sum_sum = sums[0] + sums[1] + sums[2];
    println!("Top 3 sums sum: \n{top_3_sum_sum}");
}

fn first() {
    println!("Hello, world!");

    let contents = fs::read_to_string("/Users/delikristof.demeter/Projects/adventOfCode2022/sources/1/input.txt")
        .expect("Should be able to open file");
    let mut sum = 0;
    let mut maxSum = 0;
    for a in contents.split("\n") {
        if a.eq("") {
            if sum > maxSum  { maxSum = sum; }
            sum = 0;
        } else {
            sum += a.parse::<i32>().unwrap();
        }
    }
    println!("Max sum: \n{maxSum}");
}
