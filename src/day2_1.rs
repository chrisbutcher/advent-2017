pub fn find_sum_of_evenly_divisible(input: &str) -> u32 {
  let lines = input.trim().split("\n");

  lines.fold(0, |acc, line| {
    let mut numbers: Vec<_> = line.split_whitespace().map( |n| n.parse::<u32>().unwrap() ).collect();
    numbers.sort_by( |a, b| b.cmp(a) );
    let sorted_desc_numbers = numbers;

    let mut line_total = 0;

    for (i, num_outer) in sorted_desc_numbers.iter().enumerate() {
      for (j, num_inner) in sorted_desc_numbers.iter().enumerate() {
        if i == j { continue };

        if num_outer % num_inner == 0 {
          line_total += num_outer / num_inner
        }
      }
    }

    acc + line_total
  })
}

#[test]
fn example_1() {
    let input = "5 9 2 8\n9 4 7 3\n3 8 6 5\n";
    assert_eq!(9, find_sum_of_evenly_divisible(input));
}

#[test]
fn find_sum_solution() {
    let input = super::file::read_to_string("day2-1.txt");
    assert_eq!(261, find_sum_of_evenly_divisible(input.as_str().trim()));
}
