use std::fs;

fn main() {
    // first();
    second();
}

fn first() {
    let contents = fs::read_to_string("/Users/delikristof.demeter/Projects/adventOfCode2022/sources/5/input.txt")
        .expect("Should be able to open file");
    let mut field: Vec<Vec<&str>> = vec![Vec::new(); 9];
    let mut line_num = -1;
    for a in contents.split("\n") {
        line_num += 1;
        if line_num < 8 {
            for i in 0..9 {
                let index = 1 + i * 4;
                if index < a.len() {
                    let element = &a[index..index + 1];
                    if !element.eq(" ") {
                        field.get_mut(i).as_mut().expect("Should have").push(element);
                    }
                }
            }
        } else if line_num == 8 {
            field.iter_mut().for_each(|v| v.reverse());
        } else if line_num > 9 {
            let b = a.split(" ").collect::<Vec<&str>>();
            let mvnum: i32 = b.get(1).expect("").parse().unwrap();
            let from = (b.get(3).expect("").parse::<i32>().unwrap() - 1) as usize;
            let to = (b.get(5).expect("").parse::<i32>().unwrap() - 1) as usize;
            for _k in 0..mvnum {
                let popped = field.get_mut(from).as_mut().expect("b").pop().expect("pop");
                field.get_mut(to).as_mut().expect("a").push(popped);
            }
        }
    }
    //Get top of each stack
    let result : Vec<&str> = field.iter_mut()
        .map(|v| v.pop())
        .filter(|e| !e.is_none())
        .map(|e| e.unwrap())
        .collect();
    println!("First task: {}",result.join(""))
}

fn  second() {
    let contents = fs::read_to_string("/Users/delikristof.demeter/Projects/adventOfCode2022/sources/5/input.txt")
        .expect("Should be able to open file");
    let mut field: Vec<Vec<&str>> = vec![Vec::new(); 9];
    let mut line_num = -1;
    for a in contents.split("\n") {
        line_num += 1;
        if line_num < 8 {
            for i in 0..9 {
                let index = 1 + i * 4;
                if index < a.len() {
                    let element = &a[index..index + 1];
                    if !element.eq(" ") {
                        field.get_mut(i).as_mut().expect("Should have").push(element);
                    }
                }
            }
        } else if line_num == 8 {
            field.iter_mut().for_each(|v| v.reverse());
        } else if line_num > 9 {
            let b = a.split(" ").collect::<Vec<&str>>();
            let mvnum: i32 = b.get(1).expect("").parse().unwrap();
            let from = (b.get(3).expect("").parse::<i32>().unwrap() - 1) as usize;
            let to = (b.get(5).expect("").parse::<i32>().unwrap() - 1) as usize;
            let mut battery = Vec::new();
            for _k in 0..mvnum {
                let popped = field.get_mut(from).as_mut().expect("b").pop().expect("pop");
                battery.push(popped);
            }
            for _l in 0..mvnum {
                field.get_mut(to).as_mut().expect("a").push(battery.pop().expect("pop2"));
            }
        }
    }
    //Get top of each stack
    let result : Vec<&str> = field.iter_mut()
        .map(|v| v.pop())
        .filter(|e| !e.is_none())
        .map(|e| e.unwrap())
        .collect();
    println!("Second task: {}",result.join(""))
}

// fn iterate(crane_logic: impl FnOnce(i32, usize, usize, & Vec<Vec<&str>>) -> () ) {
//     let contents = fs::read_to_string("/Users/delikristof.demeter/Projects/adventOfCode2022/sources/5/input.txt")
//         .expect("Should be able to open file");
//     let mut field: Vec<Vec<&str>> = vec![Vec::new(); 9];
//     let mut line_num = -1;
//     for a in contents.split("\n") {
//         line_num += 1;
//         if line_num < 8 {
//             for i in 0..9 {
//                 let index = 1 + i * 4;
//                 if index < a.len() {
//                     let element = &a[index..index + 1];
//                     if !element.eq(" ") {
//                         // let mut c =field.get_mut(0).as_mut().expect("");
//                         field.get_mut(i).as_mut().expect("Should have").push(element);
//                         // field.get(i).expect("Should have").push(element);
//                         // field.get(i).expect("Should have").push(element);
//                     }
//                 }
//             }
//         } else if line_num == 8 {
//             field.iter_mut().for_each(|v| v.reverse());
//         } else if line_num > 9 {
//             let b = a.split(" ").collect::<Vec<&str>>();
//             let mvnum: i32 = b.get(1).expect("").parse().unwrap();
//             let from = (b.get(3).expect("").parse::<i32>().unwrap() - 1) as usize;
//             let to = (b.get(5).expect("").parse::<i32>().unwrap() - 1) as usize;
//             crane_logic(mvnum, from, to, &field);
//             for _k in 0..mvnum {
//                 let popped = field.get_mut(from).as_mut().expect("b").pop().expect("pop");
//                 field.get_mut(to).as_mut().expect("a").push(popped);
//             }
//         }
//     }
//     //Get top of each stack
//     let result : Vec<&str> = field.iter_mut()
//         .map(|v| v.pop())
//         .filter(|e| !e.is_none())
//         .map(|e| e.unwrap())
//         .collect();
//     println!("First task: {}",result.join(""))
// }