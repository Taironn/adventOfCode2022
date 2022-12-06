use std::collections::HashSet;
use std::fs;
use std::slice::SliceIndex;

fn  main() {
    // first();
    second();
    // println!("{}",'a' as u32); //-96
    // println!("{}",'b' as u32);
    // println!("{}",'A' as u32); //-38
    // println!("{}",'B' as u32);
}

fn  second() {
    let contents = fs::read_to_string("/Users/delikristof.demeter/Projects/adventOfCode2022/sources/3/input.txt")
        .expect("Should be able to open file");
    let  mut sum = 0;
    let  elves: Vec<&str> = contents.split("\n").collect();
    for i in 0 .. elves.len()  {
        if i % 3 == 2 {
            let  set1:HashSet<char> = HashSet::from_iter(elves.get(i-2).expect("...").chars());
            let  set2:HashSet<char> = HashSet::from_iter(elves.get(i-1).expect("...").chars());
            let  set3:HashSet<char> = HashSet::from_iter(elves.get(i).expect("...").chars());
            let  char = set1.iter()
                .filter(|b| set2.contains(b))
                .filter(|c| set3.contains(c))
                .next()
                .expect("Should have 1");
            let  val = *char as u32;
            let  addval = if val > 96 {val - 96} else { val - 38 };
            sum += addval;
        }
    }
    println!("Task 2 final sum: {}", sum)
}

fn  first() {
    let contents = fs::read_to_string("/Users/delikristof.demeter/Projects/adventOfCode2022/sources/3/input.txt")
        .expect("Should be able to open file");
    let  mut sum = 0;
    for a in contents.split("\n") {
        let len = a.len();
        let  f1 = &a[..(len/2)];
        let  f2 = &a[(len/2)..];
        let  set1:HashSet<char> = HashSet::from_iter(f1.chars());
        let  set2:HashSet<char> = HashSet::from_iter(f2.chars());
        // let  intersection : HashSet<char> = set1.intersection(&set2).next();
        let  intersection : char = set1.intersection(&set2).next().expect("").clone();
        // let  result = intersection.iter().next().expect("should have one intersection!");
        let  val =intersection as u32;
        let  addval = if val > 96 {val - 96} else { val - 38 };
        sum += addval;
    }
    println!("Final sum: {}", sum)
}