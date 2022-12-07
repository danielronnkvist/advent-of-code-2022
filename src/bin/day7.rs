use std::collections::HashMap;
use std::str::FromStr;

fn iu_map(map: &mut HashMap<String, u32>, dir: String, val: u32) {
    if let Some(v) = map.get_mut(&dir) {
        *v += val;
    } else {
        map.insert(dir, val);
    }
}

fn main() {
    let input = include_str!("./../../inputs/day7.txt").trim();
    let mut visited = HashMap::new();
    let mut current_directory = vec![];
    for line in input.split("\n") {
        if line.contains("$ cd") {
            let (_, directory) = line.split_at(5);
            if directory == ".." {
                current_directory.pop();
            } else {
                current_directory.push(directory);
            }
            iu_map(&mut visited, current_directory.join("/"), 0);
        } else if line.contains("$ ls") {
            continue;
        } else {
            if !line.contains("dir") {
                iu_map(&mut visited, current_directory.join("/"), u32::from_str(line.split(" ").nth(0).unwrap()).unwrap());
            }
        }
    }

    let v = visited.clone();
    for (dir1, v1) in visited.iter_mut() {
        for (dir2, v2)  in v.iter() {
            if dir1 != dir2 && dir2.contains(dir1) {
                *v1 += *v2;
            }
        }
    }

    let result: u32 = visited.iter().filter_map(|(_, s)| {
        if s < &100000 {
            Some(s)
        } else {
            None
        }
    }).sum();

    let space = 30000000 - (70000000 - visited.get("/").unwrap());
    let mut min = *visited.get("/").unwrap();
    for (dir, s) in visited {
        if s > space && s < min {
            min = s;
        }
    }

    dbg!(result);
    dbg!(min);
}
