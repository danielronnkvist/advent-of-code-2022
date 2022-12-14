use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = include_str!("./../../inputs/day5.txt");
    let (crates, operations) = input.split_at(input.find("\n\n").unwrap());
    
    let mut crates: Vec<&str> = crates.split("\n").collect();
    crates.reverse();
    let mut crates_map: HashMap<u32, Vec<char>> = HashMap::new();
    crates[0].split(" ").filter_map(|i| u32::from_str(i).ok()).for_each(|i| {crates_map.insert(i, vec![]);});
    for i in 1..crates.len() {
        let c = crates[i];
        let mut ci = 0;
        while let Some(id) = c.chars().nth(1 + ci * 4) {
            if id != ' ' {
                if let Some(x) = crates_map.get_mut(&(1 + ci as u32)) {
                    x.push(id);
                }
            }
            ci += 1;
        }
    }

    let mut p1 = crates_map.clone();
    let mut p2 = crates_map;

    for operation in operations.split("\n") {
        if operation.is_empty() {
            continue;
        }
        let operation: Vec<&str> = operation.split(" ").collect();
        let amount = u32::from_str(operation[1]).unwrap() as usize;
        let from = u32::from_str(operation[3]).unwrap();
        let to = u32::from_str(operation[5]).unwrap();

        for _ in 0..amount {
            let id = p1.get_mut(&from).unwrap().pop().unwrap();
            p1.get_mut(&to).unwrap().push(id);
        }

        let from_vec = p2.get_mut(&from).unwrap();
        let mut taken: Vec<_> = from_vec.split_off(from_vec.len() - amount);
        p2.get_mut(&to).unwrap().append(&mut taken);
    }

    let mut result1: String = String::new();
    for i in 1..=p1.len() {
        result1 += &p1.get(&(i as u32)).unwrap().last().unwrap().to_string();
    }
    dbg!(result1);

    let mut result2: String = String::new();
    for i in 1..=p2.len() {
        result2 += &p2.get(&(i as u32)).unwrap().last().unwrap().to_string();
    }
    dbg!(result2);
}
