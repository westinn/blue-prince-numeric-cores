use std::{fs, io};

use super::numeric_core_state::states::{InvalidStateError, NumericCoreState};

// ===============================================
// Types
// ===============================================

pub enum FileParseError {
    // wrap the standard library IO error
    Io(io::Error),
    InputFileEmptyError(String),
}

impl From<io::Error> for FileParseError {
    fn from(value: io::Error) -> Self {
        FileParseError::Io(value)
    }
}

// ===============================================
// Main Logic
// ===============================================

// ===============================================
// Inidividual word parsing
// ===============================================

// file word -> valid string
fn file_word_to_string(word: &str) -> Result<String, InvalidStateError> {
    match word.chars().any(|c: char| !c.is_ascii_alphabetic()) {
        true => {
            eprintln!("Unable to parse word in file into valid ascii string: {word}.");
            Err(InvalidStateError)
        }
        false => Ok(word.to_owned()),
    }
}

// string -> number
fn string_to_number(word: &str) -> Result<u32, InvalidStateError> {
    word.chars()
        .map(|c| ((c.to_ascii_lowercase() as u32) - ('a' as u32) + 1).to_string())
        .collect::<String>()
        .parse::<u32>()
        .map_err(|parse_int_err| {
            eprintln!(
                "Unable to convert word to u32 number: {word}. Original error: {parse_int_err}"
            );
            InvalidStateError
        })
}

// number to numeric core state
fn number_to_state(num: u32) -> NumericCoreState {
    NumericCoreState::new(num)
}

// ===============================================
// loop through entire file
// ===============================================

// parse file initially
pub fn get_file_contents(cypher_file_path: &str) -> Result<String, FileParseError> {
    let file_content = fs::read_to_string(cypher_file_path)?.trim().to_owned();

    if file_content.is_empty() {
        Err(FileParseError::InputFileEmptyError(format!(
            "Input file was empty: {cypher_file_path}"
        )))
    } else {
        Ok(file_content)
    }
}

fn parse_file(file_path: &str) -> Result<String, FileParseError> {
    let file_contents: Result<String, FileParseError> = get_file_contents(file_path);

    let strings = file_to_strings(file_contents);

    todo!();
}

fn file_to_strings(file_contents: &str) -> Vec<Result<String, InvalidStateError>> {
    file_contents
        .split_ascii_whitespace()
        .map(file_word_to_string)
        .collect()
}

fn strings_to_numbers(
    string_cypher: &[Result<String, InvalidStateError>],
) -> Vec<Result<u32, InvalidStateError>> {
    string_cypher.iter().map(word_to_number).collect()
}

fn numbers_to_states(numeric_cypher: &[Result<u32, InvalidStateError>]) -> Vec<NumericCoreState> {
    // parses all numbers into NumericCoreStates
    numeric_cypher.iter().map(number_to_state).collect()
}

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
