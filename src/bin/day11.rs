use std::str::FromStr;

#[derive(Debug, Clone)]
enum Operand {
    Add,
    Multiply,
}

impl Operand {
    fn eval(&self, a: u64, b: u64) -> u64 {
        match &self {
            Self::Add => a + b,
            Self::Multiply => a * b,
        }
    }
}

impl From<&str> for Operand {
    fn from(val: &str) -> Self {
        match val.trim() {
            "+" => Self::Add,
            "*" => Self::Multiply,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone)]
enum OperationValue {
    Old,
    Value(u64)
}

impl OperationValue {
    fn get(&self, old: u64) -> u64 {
        match &self {
            Self::Old => old,
            Self::Value(v) => *v,
        }
    }
}

impl From<&str> for OperationValue {
    fn from(val: &str) -> Self {
        match val {
            "old" => Self::Old,
            _ => Self::Value(u64::from_str(val.trim()).unwrap())
        }
    }
}

#[derive(Debug, Clone)]
struct Operation {
    a: OperationValue,
    operand: Operand,
    b: OperationValue,
}

impl Operation {
    fn eval(&self, old: u64) -> u64 {
        let a = self.a.get(old);
        let b = self.b.get(old);
        self.operand.eval(a, b)
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    inspections: usize,
    items: Vec<u64>,
    operation: Operation,
    test_divisible_by: u64,
    if_true: usize,
    if_false: usize,
}

fn solve(mut monkeys: Vec<Monkey>, iterations: usize) -> usize {
    let p: u64 = monkeys.iter().map(|m| m.test_divisible_by).product();
    for _ in 0..iterations {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop() {
                monkeys[i].inspections += 1;
                let ni = monkeys[i].operation.eval(item) % p;
                let d = ni % monkeys[i].test_divisible_by;
                if d == 0 {
                    let if_true = monkeys[i].if_true;
                    monkeys[if_true].items.push(ni);
                } else {
                    let if_false = monkeys[i].if_false;
                    monkeys[if_false].items.push(ni);
                }
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));

    monkeys[0].inspections * monkeys[1].inspections
}

fn main() {
    let input = include_str!("./../../inputs/day11.txt").trim();
    let monkeys: Vec<Monkey> = input.split("\n\n").map(|monkey| {
        let lines: Vec<&str> = monkey.split("\n").collect();
        let (_, items) = lines[1].split_at(18);
        let items: Vec<u64> = items.split(",").map(|i| u64::from_str(i.trim()).unwrap()).collect();

        let (_, op) = lines[2].split_at(19);
        let op: Vec<&str> = op.split(" ").collect();
        let operation: Operation = Operation {
            a: op[0].into(),
            operand: op[1].into(),
            b: op[2].into(),
        };

        let (_, test) = lines[3].split_at(21);
        let test_divisible_by = u64::from_str(test.trim()).unwrap();

        let (_, monkey) = lines[4].split_at(29);
        let if_true = usize::from_str(monkey.trim()).unwrap();
        let (_, monkey) = lines[5].split_at(29);
        let if_false = usize::from_str(monkey.trim()).unwrap();
        
        Monkey {
            inspections: 0,
            items,
            operation,
            test_divisible_by,
            if_true,
            if_false,
        }
    }).collect();

    let p1 = solve(monkeys.clone(), 20);
    let p2 = solve(monkeys.clone(), 10000);
    dbg!(p1);
    dbg!(p2);
}
