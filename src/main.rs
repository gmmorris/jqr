#[macro_use]
extern crate lazy_static;
extern crate regex;

use clap::{App, Arg};
use json::*;
use std::io::Error;
use std::result::Result;
use std::string::String;

mod input;
mod selection;

fn match_line(
    matchers: &Vec<Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>>>,
    line: Result<String, Error>,
) {
    let input = line.expect("Could not read line from standard in");
    let json_input = json::parse(&input);
    if json_input.is_ok() {
        let json_input = json_input.unwrap();
        if match_json_slice(matchers, &json_input).is_ok() {
            println!("{}", json::stringify(json_input));
        }
    }
}

fn match_json_slice(
    matchers: &Vec<Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>>>,
    json_input: &JsonValue,
) -> Result<(), ()> {
    match json_input {
        JsonValue::Object(_) => match matchers
            .iter()
            .try_fold(json_input, |json_slice, matcher| matcher(Some(&json_slice)))
        {
            Some(_) => Ok(()),
            None => match json_input {
                JsonValue::Object(ref object) => match object
                    .iter()
                    .find(|(_, value)| match_json_slice(matchers, *value).is_ok())
                {
                    Some(_) => Ok(()),
                    None => Err(()),
                },
                _ => Err(()),
            },
        },
        _ => Err(()),
    }
}

fn verbose(filter: &str) {
    println!("filter: {}", filter);
    println!("-----");
}

fn main() {
    let matches = App::new("jgrep")
        .version("0.0.1")
        .author("Gidi Meir Morris <gidi@gidi.io>")
        .about("jgrep searches for PATTERNS in json input, jgrep prints each json object that matches a pattern.")
        .arg(
            Arg::with_name("filter")
                .required(true)
                .takes_value(true)
                .multiple(true)
                .help("JSON query filter")
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .takes_value(true)
                .help("JSON input file")
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .help("Sets the level of verbosity")
        )
        .get_matches();

    let filter = matches.value_of("filter").unwrap();

    if matches.is_present("v") {
        verbose(filter);
    }

    input::match_input(
        matches.value_of("input"),
        selection::match_filters(filter),
        &match_line,
    );
}
