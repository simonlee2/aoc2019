use std::fs;

pub fn solve() -> u32 {
    let input_content = fs::read_to_string("src/day3/input.txt").unwrap();
    let mut lines = input_content.lines();
    let line_a = lines.next().unwrap();
    let line_b = lines.next().unwrap();
    least_combined_steps(line_a.to_string(), line_b.to_string())
}

fn compute_distance(wire_a: String, wire_b: String) -> u32 {
    let line_a: Vec<Point> = create_lines(parse_wire(wire_a));
    let line_b: Vec<Point> = create_lines(parse_wire(wire_b));
    let common_points = intersect(&line_a, &line_b);
    let min_distance = common_points
        .iter()
        .map(|point| Point { x: 0, y: 0 }.manhattan_distance(point))
        .min()
        .unwrap();

    min_distance
}

fn least_combined_steps(wire_a: String, wire_b: String) -> u32 {
    let line_a: Vec<Point> = create_lines(parse_wire(wire_a));
    let line_b: Vec<Point> = create_lines(parse_wire(wire_b));
    let common_points = intersect(&line_a, &line_b);
    common_points
        .iter()
        .map(|point| compute_steps(&line_a, point) + compute_steps(&line_b, point) + 2)
        .min()
        .unwrap()
}

fn compute_steps(wire: &Vec<Point>, point: &Point) -> u32 {
    wire.iter().position(|&x| x == *point).unwrap() as u32
}

fn intersect(line: &Vec<Point>, other: &Vec<Point>) -> Vec<Point> {
    let mut out = vec![];
    for x in line {
        for y in other {
            if x == y {
                println!("Comparing {:?} to {:?}", x, y);
                out.push(x.clone());
                break;
            }
        }
    }

    println!("common points = {:?}", out);
    out
}

#[derive(Debug)]
enum Op {
    Right(usize),
    Left(usize),
    Up(usize),
    Down(usize),
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self, other: &Self) -> u32 {
        ((other.x - self.x).abs() + (other.y - self.y).abs()) as u32
    }
}

impl PartialEq for Point {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

fn create_lines(ops: Vec<Op>) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];
    for op in ops {
        let prev_point = points.last().unwrap_or(&Point { x: 0, y: 0 });
        let new_points = points_from(prev_point, op);
        points.extend(new_points);
    }

    points
}

fn points_from(point: &Point, op: Op) -> Vec<Point> {
    match op {
        Op::Right(l) => (point.x + 1..point.x + (l as i32) + 1)
            .map(|x| Point { x: x, y: point.y })
            .collect(),
        Op::Left(l) => ((point.x - (l as i32))..point.x)
            .rev()
            .map(|x| Point { x: x, y: point.y })
            .collect(),
        Op::Down(l) => ((point.y - (l as i32))..point.y)
            .rev()
            .map(|y| Point { x: point.x, y: y })
            .collect(),
        Op::Up(l) => (point.y + 1..point.y + (l as i32) + 1)
            .map(|y| Point { x: point.x, y: y })
            .collect(),
    }
}

fn lex(input: String) -> Vec<String> {
    input.clone().split(",").map(|s| String::from(s)).collect()
}

fn parse_op(op_string: String) -> Option<Op> {
    let (instruction, length) = op_string.split_at(1);
    let length: usize = length.parse().unwrap();
    match instruction {
        "R" => Some(Op::Right(length)),
        "L" => Some(Op::Left(length)),
        "U" => Some(Op::Up(length)),
        "D" => Some(Op::Down(length)),
        _ => None,
    }
}
fn parse_wire(wire: String) -> Vec<Op> {
    let tokens = lex(wire);
    tokens
        .iter()
        .map(|token| parse_op(token.to_string()).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample1() {
        let wire_a = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire_b = String::from("U62,R66,U55,R34,D71,R55,D58,R83");
        let distance = compute_distance(wire_a, wire_b);
        assert_eq!(distance, 159);
    }

    #[test]
    fn test_sample2() {
        let wire_a = String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let wire_b = String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

        let distance = compute_distance(wire_a, wire_b);
        assert_eq!(distance, 135);
    }

    #[test]
    fn test_line_right() {
        let op = Op::Right(3);
        let points = points_from(&Point { x: 0, y: 0 }, op);
        assert_eq!(
            points,
            vec![
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 }
            ]
        )
    }

    #[test]
    fn test_line_down() {
        let op = Op::Down(3);
        let points = points_from(&Point { x: 0, y: 10 }, op);
        assert_eq!(
            points,
            vec![
                Point { x: 0, y: 9 },
                Point { x: 0, y: 8 },
                Point { x: 0, y: 7 }
            ]
        )
    }

    #[test]
    fn test_line_left() {
        let op = Op::Left(3);
        let points = points_from(&Point { x: 10, y: 10 }, op);
        assert_eq!(
            points,
            vec![
                Point { x: 9, y: 10 },
                Point { x: 8, y: 10 },
                Point { x: 7, y: 10 }
            ]
        )
    }

    #[test]
    fn test_line_up() {
        let op = Op::Up(3);
        let points = points_from(&Point { x: 0, y: 0 }, op);
        assert_eq!(
            points,
            vec![
                Point { x: 0, y: 1 },
                Point { x: 0, y: 2 },
                Point { x: 0, y: 3 }
            ]
        )
    }

    #[test]
    fn test_line() {
        let ops = vec![Op::Up(3), Op::Right(3)];
        let points = create_lines(ops);
        assert_eq!(
            points,
            vec![
                Point { x: 0, y: 1 },
                Point { x: 0, y: 2 },
                Point { x: 0, y: 3 },
                Point { x: 1, y: 3 },
                Point { x: 2, y: 3 },
                Point { x: 3, y: 3 },
            ]
        )
    }

    #[test]
    fn test_least_steps_1() {
        let wire_a = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire_b = String::from("U62,R66,U55,R34,D71,R55,D58,R83");
        let distance = least_combined_steps(wire_a, wire_b);
        assert_eq!(distance, 610);
    }

    #[test]
    fn test_least_steps_2() {
        let wire_a = String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let wire_b = String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let distance = least_combined_steps(wire_a, wire_b);
        assert_eq!(distance, 410);
    }
}
