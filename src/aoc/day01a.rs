use std::path::Path;
use crate::aoc::file;

fn solve() -> i32 {
    solve_file (Path::new("../../resources/input/input01.txt"))
}

fn solve_file(f:&Path) -> i32{
    let numbers = read_input(f);
    let count = count_increase(&numbers);
    count
}

fn read_input(f:&Path) -> Vec<i32> {
    let lines = file::read_lines(f).unwrap();
    let mut numbers = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let number:i32 = line.parse().unwrap();
        numbers.push(number);

    };
    numbers
}

fn count_increase(numbers: &Vec<i32>) ->i32{
    let mut incs = 0;
    let mut now = numbers[0];
    for num in numbers {
        if num > &now {
            incs = incs + 1;
        }
        now=*num;
    };
    incs
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let result = solve();
        println!("result : {}", result);
        assert_eq!(result,1692);
    }


}