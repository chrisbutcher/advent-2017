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

  let square_width = square_width;

  let mut target_tuple: (i32, i32) = (0, 0);
  let mut done = false;

  let mut grid = vec![vec![0; square_width as usize]; square_width as usize];
  let mut current_direction = Direction::Left;

  let mut y = square_width - 1;
  let mut x = square_width - 1;

  let mut i = square_width.pow(2);
  grid[y as usize][x as usize] = i;

  'a: for _ in 0..3 {
    for _ in 0..(square_width - 1) {
      let (next_y, next_x) = direction_to_tuple(y, x, &current_direction);

      y = next_y;
      x = next_x;
      i -= 1;
      grid[y as usize][x as usize] = i;

      if i == input {
        target_tuple = (y as i32, x as i32);
        done = true;
        break 'a;
      }

    }
    current_direction = direction_to_next_direction(&current_direction);
  }

  if !done {
    'b: loop {
      for _ in 0..(square_width - 1) {
        let (next_y, next_x) = direction_to_tuple(y, x, &current_direction);

        let grid_clone = grid.clone();
        let allowed = match grid_clone[next_y as usize].get(next_x as usize) {
          Some(n) => {
            if *n == 0 {
              true
            } else {
              false
            }
          }
          _ => false
        };

        if allowed {
          y = next_y;
          x = next_x;
          i -= 1;

          grid[y as usize][x as usize] = i;

          if i == input {
            target_tuple = (y as i32, x as i32);
            break 'b;
          } else if i == 1 {
            break 'b;
          }
        } else {
          break;
        }
      }
      current_direction = direction_to_next_direction(&current_direction);
    }
  }

  let center: (i32, i32) = (square_width as i32 / 2, square_width as i32 / 2);
  let result = (target_tuple.0 - center.0).abs() + (target_tuple.1 - center.1).abs();

  result as u32
}

fn direction_to_next_direction(direction: &Direction) -> Direction {
  match *direction {
    Direction::Right => Direction::Down,
    Direction::Up => Direction::Right,
    Direction::Left => Direction::Up,
    Direction::Down => Direction::Left,
  }
}

fn direction_to_tuple(y: u32, x: u32, direction: &Direction) -> (u32, u32) {
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
  assert_eq!(438, find_steps_from(265149));
}
