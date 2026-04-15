use std::{
    fs::{self},
    io, num,
};

use crate::numeric_core_solver::numeric_core_state;

use super::numeric_core_state::states::NumericCoreState;

// ===============================================
// Types
// ===============================================

#[derive(Debug, Clone)]
pub struct CypherToken {
    pub string_value: Result<String, FileParseError>,
    pub numeric_value: Result<u32, FileParseError>,
    pub state: NumericCoreState,
}

#[derive(Debug, Clone)]
pub enum FileParseError {
    // wrap the standard library IO error
    Io(String),
    InputFileEmptyError(String),
    NonAsciiWord(String),
    U32ParseError(num::ParseIntError),
}

impl From<io::Error> for FileParseError {
    fn from(value: io::Error) -> Self {
        FileParseError::Io(value.to_string())
    }
}

impl From<num::ParseIntError> for FileParseError {
    fn from(value: num::ParseIntError) -> Self {
        FileParseError::U32ParseError(value)
    }
}

// ===============================================
// Main Logic
// ===============================================

// ===============================================
// Inidividual word parsing
// ===============================================

// file word -> valid string
fn file_word_to_string(word: &str) -> Result<String, FileParseError> {
    match word.chars().any(|c: char| !c.is_ascii_alphabetic()) {
        true => {
            let err_message =
                format!("Unable to parse word in file into valid ascii string: {word}.");
            eprintln!("{}", err_message);
            Err(FileParseError::NonAsciiWord(err_message.to_owned()))
        }
        false => Ok(word.to_owned()),
    }
}

// string -> number
fn string_to_number(word: &str) -> Result<u32, FileParseError> {
    word.chars()
        .map(|c| ((c.to_ascii_lowercase() as u32) - ('a' as u32) + 1).to_string())
        .collect::<String>()
        .parse::<u32>()
        .map_err(Into::into)
}

// number to numeric core state
fn number_to_state(num: Option<u32>) -> NumericCoreState {
    NumericCoreState::new(num)
}

// ===============================================
// loop through entire file
// ===============================================

fn file_to_cypher(cypher_file_path: &str) -> Result<Vec<NumericCoreState>, FileParseError> {
    let token_results: Vec<CypherToken> = get_file_contents(cypher_file_path)?
        .split_ascii_whitespace()
        .map(|file_word| -> CypherToken {
            let word: Result<String, FileParseError> = file_word_to_string(file_word);

            let numeric_value: Result<u32, FileParseError> = string_to_number(word);

            let state: NumericCoreState = NumericCoreState::new(numeric_value);

            CypherToken {
                string_value: word,
                numeric_value: numeric_value,
                state: state,
            }
        })
        .collect();

    todo!();
}

// parse file initially
pub fn get_file_contents(cypher_file_path: &str) -> Result<String, FileParseError> {
    let file_content = fs::read_to_string(cypher_file_path)?.trim().to_owned();

    match file_content.is_empty() {
        true => Err(FileParseError::InputFileEmptyError(format!(
            "Input file was empty: {cypher_file_path}"
        ))),
        false => Ok(file_content),
    }
}

// fn file_to_strings(file_contents: &str) -> Vec<Result<String, InvalidStateError>> {
//     file_contents
//         .split_ascii_whitespace()
//         .map(file_word_to_string)
//         .collect()
// }

// fn strings_to_numbers(
//     string_cypher: &[Result<String, InvalidStateError>],
// ) -> Vec<Result<u32, InvalidStateError>> {
//     string_cypher
//         .iter()
//         .map(|current_string| {
//             let arg_as_ref = current_string.as_deref();
//             match arg_as_ref {
//                 Ok(value) => string_to_number(value),
//                 Err(error) => Err(error.clone()),
//             }
//         })
//         .collect()
// }

// fn numbers_to_states(numeric_cypher: &[Result<u32, InvalidStateError>]) -> Vec<NumericCoreState> {
//     // parses all numbers into NumericCoreStates
//     numeric_cypher
//         .iter()
//         .map(|current_number| {
//             let arg_as_ref = current_number;
//             match arg_as_ref {
//                 Ok(value) => number_to_state(*value),
//                 Err(_) => NumericCoreState::Invalid,
//             }
//         })
//         .collect()
// }

fn compute_cypher_structure(file_contents: &str) -> Result<(usize, usize), String> {
    /*
    1 2 3
    1 2 3 */
    let y_cypher_rows: usize = file_contents.lines().count(); // 2
    let x_cypher_longest_row: usize = file_contents // 3
            .lines()
            .max_by_key(|line| line.split_ascii_whitespace().count())
            .ok_or("Could not find longest row in cypher. Error occurred during initial cypher creation during parsing.")?
            .split_ascii_whitespace()
            .count();
    // (x, y)
    Ok((x_cypher_longest_row, y_cypher_rows))
}
