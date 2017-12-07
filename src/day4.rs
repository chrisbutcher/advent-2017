use std::collections::HashSet;

pub fn count_valid_passphrases(input: &str) -> u32 {
  let lines: Vec<_> = input.split("\n").collect();

  let mut valid_passphrases = 0;
  let mut unique_words = HashSet::new();
  let mut passphrase_valid;

  for line in lines {
    let words: Vec<_> = line.split(" ").collect();

    passphrase_valid = true;
    for word in words {
      if unique_words.contains(word) {
        passphrase_valid = false;
        break;
      } else {
        unique_words.insert(word);
      }
    }

    if passphrase_valid {
      valid_passphrases += 1;
    }

    unique_words.clear();
  }

  valid_passphrases
}

#[test]
fn example_1() {
  let input = "aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa\n";
  assert_eq!(2, count_valid_passphrases(input.trim()));
}

#[test]
fn find_solution() {
  let input = super::file::read_to_string("day4.txt");
  assert_eq!(466, count_valid_passphrases(input.as_str().trim()));
}
