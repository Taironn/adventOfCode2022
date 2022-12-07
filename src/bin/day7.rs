use std::collections::HashMap;
use std::fs;

fn main() {
    first();
}

fn first() {
    // let contents = fs::read_to_string("/Users/delikristof.demeter/Projects/adventOfCode2022/sources/7/test1.txt")
    let contents = fs::read_to_string("/Users/delikristof.demeter/Projects/adventOfCode2022/sources/7/input.txt")
        .expect("Should be able to open file");
    let mut pwd = String::new();
    let mut dir = HashMap::new();
    for a in contents.split("\n") {
        if a.starts_with("$") {
            let b = a.split(" ").collect::<Vec<&str>>();
            if b[1].eq("ls") { continue; }
            let cd = b[2];
            if cd.eq("/") {
                pwd = "$".parse().unwrap();
            } else if cd.eq("..") {
                pwd = pwd[0..pwd.rfind("/").unwrap()].parse().unwrap();
            } else {
                pwd = pwd.to_owned() + "/" + cd.clone();
                dir.insert(pwd.to_string(), 0 as i64);
            }
        } else if a.starts_with("dir") {
            continue;
        } else {
            let siz: i64 = (a.split(" ").collect::<Vec<&str>>())[0].parse().unwrap();
            let cur = dir.get(&*pwd);
            if cur.is_none() {
                dir.insert(pwd.to_string(), siz);
            } else { dir.insert(pwd.to_string(), cur.unwrap() + siz); }
        }
    }
    let mut dir_final = HashMap::new();
    for el in &dir {
        let  total = dir.iter()
            .filter(|d| (*d).0.starts_with(el.0))
            .map(|d| d.1)
            .sum();
        dir_final.insert(el.0.to_string(), total);
        println!("{}   {}", el.0, total)
    }
    let result: i64 = dir_final.values()
        .filter(|n| **n <= (100000 as i64))
        .sum();
    println!("First task: {}", result);

    //total:  70000000
    //used:   42476859
    //update: 30000000
    //needed:  2476859
    let  needed: &i64 = dir_final.values()
        .filter(|n| **n >= (2476859 as i64))
        .min().unwrap();
    println!("Second task: {}", needed);
}