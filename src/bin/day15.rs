use std::{collections::HashSet, ops::RangeInclusive, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

struct Sensor {
    sensor: Position,
    beacon: Position,
}

impl Sensor {
    fn radius_to(&self, to: &Position) -> i64 {
        let x = (to.x - self.sensor.x).abs();
        let y = (to.y - self.sensor.y).abs();

        x + y
    }

    fn get_y_range(&self) -> RangeInclusive<i64> {
        let radius = self.radius_to(&self.beacon);
        (self.sensor.y - radius)..=(self.sensor.y + radius)
    }

    fn get_for_y(&self, row: i64) -> HashSet<(i64, i64)> {
        let mut result = HashSet::new();
        let radius = self.radius_to(&self.beacon);
        let y_range = (self.sensor.y - radius)..=(self.sensor.y + radius);
        if y_range.contains(&row) {
            let dx = radius - (self.sensor.y - row).abs();
            for x in (self.sensor.x - dx)..=(self.sensor.x + dx) {
                let p = Position { x, y: row };
                if p != self.beacon && p != self.sensor {
                    result.insert((x, row));
                }
            }
        }
        result
    }

    fn contains(&self, pos: (i64, i64)) -> bool {
        let radius = self.radius_to(&self.beacon);
        let y_range = (self.sensor.y - radius)..=(self.sensor.y + radius);
        let dx = radius - (self.sensor.y - pos.1).abs();
        let x_range = (self.sensor.x - dx)..=(self.sensor.x + dx);
        x_range.contains(&pos.0) && y_range.contains(&pos.1)
    }
}

fn get_position(s: &str) -> Position {
    let pos: Vec<&str> = s.split(",").collect();
    Position {
        x: i64::from_str(pos[0].split("=").nth(1).unwrap()).unwrap(),
        y: i64::from_str(pos[1].split("=").nth(1).unwrap()).unwrap(),
    }
}

fn solve(input: &str, row: i64, search: i64) -> (i64, i64) {
    let mut sensors: Vec<Sensor> = vec![];
    for line in input.split("\n") {
        let (sensor, beacon) = line.split_at(line.chars().position(|c| c == ':').unwrap());
        let sensor = get_position(sensor);
        let beacon = get_position(beacon);
        let sensor = Sensor { sensor, beacon };
        sensors.push(sensor);
    }

    let mut result: HashSet<(i64, i64)> = HashSet::new();
    for sensor in &sensors {
        result.extend(sensor.get_for_y(row));
    }
    let result = result.len() as i64;

    let mut p2 = Some((0, 0));
    'find: for sensor in &sensors {
        let radius = sensor.radius_to(&sensor.beacon) + 1;
        for y in sensor.get_y_range() {
            if y >= 0 && y <= search {
                let dx = radius - (sensor.sensor.y - y).abs();
                let mut x_range = (sensor.sensor.x - dx)..=(sensor.sensor.x + dx);
                if x_range.start() < &0 && x_range.end() < &0 {
                    continue;
                }
                if x_range.start() > &search && x_range.end() > &search {
                    continue;
                }
                if x_range.start() < &0 {
                    x_range = 0..=*x_range.end();
                }
                if x_range.end() > &search {
                    x_range = *x_range.start()..=search;
                }

                let test = sensors.iter().all(|s| !s.contains((*x_range.start(), y)));
                if test {
                    p2 = Some((*x_range.start(), y));
                    break 'find;
                }
                let test = sensors.iter().all(|s| !s.contains((*x_range.end(), y)));
                if test {
                    p2 = Some((*x_range.end(), y));
                    break 'find;
                }
            }
        }
    }

    (result, p2.unwrap().0 * 4000000 + p2.unwrap().1)
}

fn main() {
    let ex = solve(include_str!("./../../inputs/day15-ex.txt").trim(), 10, 20);
    assert_eq!(ex.0, 26);
    assert_eq!(ex.1, 56000011);
    let input = include_str!("./../../inputs/day15.txt").trim();
    let p1 = solve(input, 2000000, 4000000);
    dbg!(p1.0);
    dbg!(p1.1);
    assert_eq!(p1.0, 4811413);
    assert_eq!(p1.1, 13171855019123);
}
