use super::file;

fn find_sum(input: &str) -> u32 {
    let input_length = input.len();

    let chars: Vec<_> = input.chars().map( |n| n.to_digit(10).unwrap() ).collect();
    let mut peekable_cycling_chars = chars.iter().cycle().peekable();

    (0..input_length).fold(0, |acc, _| {
        let value = peekable_cycling_chars.next().unwrap();
        let next_value = peekable_cycling_chars.peek().unwrap();

        if value == *next_value {
            acc + value
        } else {
            acc
        }
    })
}

#[test]
fn find_sum_example_1() {
    assert_eq!(3, find_sum("1122"));
}

#[test]
fn find_sum_example_2() {
    assert_eq!(4, find_sum("1111"));
}

#[test]
fn find_sum_example_3() {
    assert_eq!(0, find_sum("1234"));
}

#[test]
fn find_sum_example_4() {
    assert_eq!(9, find_sum("91212129"));
}

#[test]
fn find_sum_solution() {
    let input = file::read_to_string("day1-1.txt");
    assert_eq!(1029, find_sum(input.as_str().trim()));
}
