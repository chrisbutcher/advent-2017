use super::file;

fn find_checksum(input: &str) -> u32 {
  let lines = input.split("\n");

  lines.fold(0, |acc, line| {
    let numbers: Vec<_> = line.split_whitespace().map( |n| n.parse::<u32>().unwrap() ).collect();
    let min = numbers.iter().min().unwrap();
    let max = numbers.iter().max().unwrap();
    let difference = max - min;

    acc + difference
  })
}

#[test]
fn find_sum_example_1() {
    let input = "5 1 9 5\n7 5 3\n2 4 6 8";
    assert_eq!(18, find_checksum(input));
}

#[test]
fn find_sum_solution() {
    let input = file::read_to_string("day2-1.txt");
    assert_eq!(36766, find_checksum(input.as_str().trim()));
}
