use std::{cmp::Ordering, collections::VecDeque};

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Integer(u32),
    List(Vec<Value>),
}

fn validate_pair(a: Value, b: Value) -> Ordering {
    match (&a, &b) {
        (Value::Integer(a), Value::Integer(b)) => {
            if a < b {
                return Ordering::Less;
            } else if a == b {
                return Ordering::Equal;
            } else {
                return Ordering::Greater;
            }
        }
        (Value::List(a), Value::List(b)) => {
            let mut i = 0;
            while i < a.len() && i < b.len() {
                let c = validate_pair(a[i].clone(), b[i].clone());
                if c == Ordering::Less {
                    return c;
                } else if c == Ordering::Greater {
                    return c;
                }
                i += 1;
            }
            if i == a.len() && i < b.len() {
                return Ordering::Less;
            } else if i == b.len() && i < a.len() {
                return Ordering::Greater;
            } else {
                return Ordering::Equal;
            }
        }
        (Value::Integer(_), Value::List(_)) => validate_pair(Value::List(vec![a]), b),
        (Value::List(_), Value::Integer(_)) => validate_pair(a, Value::List(vec![b])),
    }
}

fn parse_line(line: &str) -> Vec<Value> {
    let mut chars: VecDeque<char> = line.chars().collect();
    let mut result: Vec<Value> = vec![];
    while let Some(c) = chars.pop_front() {
        if c == '[' {
            let mut list = String::from("");
            let mut brackets = 1;
            while let Some(sc) = chars.pop_front() {
                if sc == '[' {
                    brackets += 1;
                } else if sc == ']' {
                    brackets -= 1;
                    if brackets == 0 {
                        break;
                    }
                }
                list.push_str(&sc.to_string());
            }
            result.push(Value::List(parse_line(&list)));
        } else if let Some(_) = c.to_digit(10) {
            let mut num = c.to_string();
            while let Some(sc) = chars.pop_front() {
                if sc != ',' && sc != ']' {
                    num.push_str(&sc.to_string());
                } else {
                    break;
                }
            }

            if let Ok(i) = u32::from_str_radix(&num, 10) {
                result.push(Value::Integer(i));
            }
        }
    }
    result
}

fn parse_pair(txt: &str) -> (Value, Value) {
    let pairs: Vec<Value> = txt
        .trim()
        .split("\n")
        .map(parse_line)
        .map(Value::List)
        .collect();
    (pairs[0].clone(), pairs[1].clone())
}

fn main() {
    let input = include_str!("./../../inputs/day13-ex.txt").trim();
    let mut p0 = 0;
    for (i, lines) in input.split("\n\n").enumerate() {
        let (a, b) = parse_pair(lines);
        if validate_pair(a, b) == Ordering::Less {
            p0 += i + 1;
        }
    }
    assert_eq!(p0, 13);

    let input = include_str!("./../../inputs/day13.txt").trim();
    let mut p1 = 0;
    for (i, lines) in input.split("\n\n").enumerate() {
        let (a, b) = parse_pair(lines);
        if validate_pair(a, b) == Ordering::Less {
            p1 += i + 1;
        }
    }
    dbg!(p1);
    assert_eq!(p1, 5852);

    let input = include_str!("./../../inputs/day13.txt").trim();
    let mut values: Vec<Value> = input
        .split("\n")
        .filter_map(|l| {
            if !l.is_empty() {
                return Some(Value::List(parse_line(l)));
            }
            None
        })
        .collect();
    let dpack0 = Value::List(vec![Value::List(vec![Value::Integer(2)])]);
    values.push(dpack0.clone());
    let dpack1 = Value::List(vec![Value::List(vec![Value::Integer(6)])]);
    values.push(dpack1.clone());
    values.sort_by(|a, b| validate_pair(a.clone(), b.clone()));
    let ipack0 = 1 + values.iter().position(|e| e == &dpack0).unwrap();
    let ipack1 = 1 + values.iter().position(|e| e == &dpack1).unwrap();
    let p2 = ipack0 * ipack1;
    dbg!(p2);
    assert_eq!(p2, 24190);
}
