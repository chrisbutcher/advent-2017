pub fn find_sum(input: &str) -> u32 {
    let input_length = input.len();

    let chars: Vec<_> = input.chars().map( |n| n.to_digit(10).unwrap() ).collect();

    (0..input_length).fold(0, |acc, index| {
        let value = chars[index];

        let opposite_value_index = ((input_length / 2) + index) % input_length;
        let opposite_value = chars[opposite_value_index ];

        if value == opposite_value {
            acc + value
        } else {
            acc
        }
    })
}

#[test]
fn find_sum_example_1() {
    assert_eq!(6, find_sum("1212"));
}

#[test]
fn find_sum_example_2() {
    assert_eq!(0, find_sum("1221"));
}

#[test]
fn find_sum_example_3() {
    assert_eq!(4, find_sum("123425"));
}

#[test]
fn find_sum_example_4() {
    assert_eq!(12, find_sum("123123"));
}

#[test]
fn find_sum_example_5() {
    assert_eq!(4, find_sum("12131415"));
}

#[test]
fn find_sum_solution() {
    let input = super::file::read_to_string("day1-1.txt");
    assert_eq!(1220, find_sum(input.as_str().trim()));
}
