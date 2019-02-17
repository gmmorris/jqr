use json::JsonValue;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, BufRead, BufReader, Error};
use std::result::Result;
use std::string::String;

pub fn print_input(
  filter: Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>>,
  match_line: &Fn(&Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>>, Result<String, Error>),
) {
  let stdin = io::stdin();
  for line in stdin.lock().lines() {
    match_line(&filter, line)
  }
}

pub fn print_input_file(
  filter: Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>>,
  input: &str,
  match_line: &Fn(&Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>>, Result<String, Error>),
) {
  let file = match File::open(input) {
    Ok(contents) => contents,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => panic!("The specified input file could not be found: {:?}", input),
      other_error => panic!(
        "There was a problem opening the file '{:?}': {:?}",
        input, other_error
      ),
    },
  };
  for line in BufReader::new(file).lines() {
    match_line(&filter, line)
  }
}
