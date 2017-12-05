#[derive(Debug)]
enum Direction {
  Right,
  Up,
  Left,
  Down
}

pub fn find_steps_from(input: u32) -> u32 {
  let mut square_width: u32 = 3;

  loop {
    if input < square_width.pow(2) {
      break;
    } else {
      square_width += 2;
    }
  }

  // Re-bind as immutable usize
  let square_width = square_width as usize;

  let mut vec = vec![vec![0; square_width]; square_width];

  let mut y = square_width - 1;
  let mut x = square_width - 1;

  let mut i = square_width.pow(2);

  vec[y][x] = i;

  let current_direction = Direction::Left;

  loop {
    let (next_y, next_x) = direction_to_tuple(y, x, &current_direction);
    let vec_clone = vec.clone();
    let grid_value = vec_clone[next_y].get(next_x);

    // let change_direction = match grid_value {
    //   Some(v) => {
    //     // if *v == 0 {
    //       false
    //     // }
    //   },
    //   None => {
    //     true
    //   }
    // };

    // let current_direction = if change_direction {
    //   direction_to_next_direction(&current_direction)
    // } else {
    //   i -= 1;
    //   y = next_y;
    //   x = next_x;
    //   vec[y][x] = i;

    //   direction_identify(&current_direction)
    // };
  }

  println!("{:?}", vec);

  // TODO
  0
}

fn direction_identify(direction: &Direction) -> Direction {
  match *direction {
    Direction::Right => Direction::Right,
    Direction::Up => Direction::Up,
    Direction::Left => Direction::Left,
    Direction::Down => Direction::Down,
  }
}

fn direction_to_next_direction(direction: &Direction) -> Direction {
  match *direction {
    Direction::Right => Direction::Down,
    Direction::Up => Direction::Right,
    Direction::Left => Direction::Up,
    Direction::Down => Direction::Left,
  }
}

fn direction_to_tuple(y: usize, x: usize, direction: &Direction) -> (usize, usize) {
  match *direction {
    Direction::Right => (y, x + 1),
    Direction::Up => (y - 1, x),
    Direction::Left => (y, x - 1),
    Direction::Down => (y + 1, x),
  }
}

#[test]
fn example_1() {
  assert_eq!(3, find_steps_from(12));
}

#[test]
fn find_solution() {
  // assert_eq!(0, find_steps_from(265149));
}
