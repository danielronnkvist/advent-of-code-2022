use std::str::FromStr;

fn draw(cycle: i32, val: i32) {
    if (val..(val + 3)).contains(&(cycle % 40)) {
        print!("#");
    } else {
        print!(".");
    }

    if cycle % 40 == 0 {
        println!();
    }
}

fn main() {
    let input = include_str!("./../../inputs/day10.txt").trim();
    let mut register: Vec<i32> = vec![1];
    for line in input.split("\n") {
        let (command, val) = line.split_at(4);
        let x = *register.last().unwrap();
        draw(register.len() as i32, x);
        register.push(x);
        if command == "addx" {
            draw(register.len() as i32, x);
            let val = i32::from_str(val.trim()).unwrap();
            register.push(x + val);
        }
    }

    let result: i32 = [20, 60, 100, 140, 180, 220].iter().map(|&x| register[x - 1] * x as i32).sum();
    println!("\n{}", result);
}
