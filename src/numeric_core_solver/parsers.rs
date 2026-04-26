use itertools::Itertools;
use std::{fmt::Display, io, num};

// ===============================================
// Types
// ===============================================

#[derive(Debug, Clone)]
pub(crate) struct CypherToken {
    // unprocessed input as String
    pub raw_text: String,
    // we either parse a valid string or error
    pub(crate) string_value: Result<String, ParseError>,
    // we either parse a valid number or dont
    // but we also could have previous errors and thus no numeric_value to read from
    pub(crate) initial_digit_values: Option<Vec<u32>>,
}

impl CypherToken {
    // TODO: could flatten these types of values so None === []
    //       though the representation of never being able to SET digits in the first place might be helpful
    pub(crate) fn get_initial_digit_values(&self) -> Option<&[u32]> {
        self.initial_digit_values.as_deref()
    }
}

impl Display for CypherToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?}, {:?}, {:?}",
            self.raw_text, self.string_value, self.initial_digit_values
        )
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
// @TODO: dead code?
pub enum ParseError {
    // wrap the standard library IO error
    Io(String),
    InputEmptyError(String),
    NonAsciiWord(String),
    U32ParseError(num::ParseIntError),
    RowParseError(String),
}

impl From<io::Error> for ParseError {
    fn from(value: io::Error) -> Self {
        ParseError::Io(value.to_string())
    }
}

impl From<num::ParseIntError> for ParseError {
    fn from(value: num::ParseIntError) -> Self {
        ParseError::U32ParseError(value)
    }
}

// ===============================================
// Utilities
// ===============================================

pub(crate) fn compute_cypher_structure(input_content: &str) -> (usize, usize) {
    let y_cypher_rows = input_content.lines().count();
    let x_cypher_columns = input_content
        .lines()
        .map(|line| line.split_ascii_whitespace().count())
        .max()
        .unwrap_or(0);
    (x_cypher_columns, y_cypher_rows)
}

// ===============================================
// Main Logic
// ===============================================

// ===============================================
// Inidividual value parsing
// ===============================================

// word -> valid string
fn word_to_string(word: &str) -> Result<String, ParseError> {
    match word.chars().all(|c: char| c.is_ascii_alphabetic()) {
        true => Ok(word.to_owned()),
        false => {
            let err_message = format!("Unable to parse word into valid ascii string: {word}");
            eprintln!("{}", err_message);
            Err(ParseError::NonAsciiWord(err_message.to_owned()))
        }
    }
}

// string -> digit group of u32
fn string_to_digit_group(word: &str) -> Vec<u32> {
    word.chars()
        .map(|c| (c.to_ascii_uppercase() as u32) - ('A' as u32) + 1)
        .collect_vec()
}

// ===============================================
// larger orchestrators + related From function
// ===============================================

pub(crate) fn input_to_cypher_tokens(input_content: &str) -> Vec<CypherToken> {
    input_content
        .split_ascii_whitespace()
        .map(Into::into)
        .collect_vec()
}

// take an input's parsed word strings, validate them, then convert them to tokens
impl From<&str> for CypherToken {
    fn from(word: &str) -> Self {
        let raw_text: String = word.to_owned();
        let string_value: Result<String, ParseError> = word_to_string(&raw_text);

        let initial_digit_values: Option<Vec<u32>> = match &string_value {
            Ok(string_value) => Some(string_to_digit_group(string_value)),
            Err(_) => None,
        };

        CypherToken {
            raw_text,
            string_value,
            initial_digit_values,
        }
    }
}
