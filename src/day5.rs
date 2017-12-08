use std::collections::HashSet;

pub fn count_steps_to_exit(input: &str) -> u32 {
  let lines: Vec<_> = input.split("\n").collect();

  let mut jumps: Vec<_> = lines.iter().map( |n| n.parse::<i32>().unwrap() ).collect();
  let mut pc = 0;

  let mut steps_to_exit = 0;

  loop {
    match jumps.get_mut(pc as usize) {
      Some(jump) => {
        pc += *jump;
        *jump += 1;
        steps_to_exit += 1;
      },
      None => {
        break
      }
    }
  }

  steps_to_exit
}

#[test]
fn example_1() {
  let input = "0\n3\n0\n1\n-3";

  assert_eq!(5, count_steps_to_exit(input.trim()));
}

#[test]
fn find_solution() {
  let input = super::file::read_to_string("day5.txt");
  assert_eq!(358131, count_steps_to_exit(input.as_str().trim()));
}
