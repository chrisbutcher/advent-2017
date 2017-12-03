use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn read_to_string(filename: &str) -> String {
  let file = File::open(filename).unwrap();
  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents).unwrap();

  contents
}
