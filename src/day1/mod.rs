use std::fs;

pub fn solve() -> (i32, i32) {
    let input_file_name = "src/day1/input.txt";
    let input_content = fs::read_to_string(input_file_name).unwrap();
    let input_numbers: Vec<i32> = input_content
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let incorrect_fuel = input_numbers.clone().into_iter().map(solution_1).sum();
    let correct_fuel = input_numbers.clone().into_iter().map(solution_2).sum();

    (incorrect_fuel, correct_fuel)
}

fn solution_1(input: i32) -> i32 {
    input / 3 - 2
}

fn solution_2(input: i32) -> i32 {
    let fuel_mass = solution_1(input);
    if fuel_mass < 0 {
        return 0;
    }

    fuel_mass + solution_2(fuel_mass)
}
