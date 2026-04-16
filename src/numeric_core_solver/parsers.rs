use std::{
    fs::{self},
    io, num,
};

use itertools::Itertools;

use super::numeric_core_state::states::NumericCoreState;

// ===============================================
// Utilities
// ===============================================

// read file
fn read_file_contents(cypher_file_path: &str) -> Result<String, FileParseError> {
    let file_content = fs::read_to_string(cypher_file_path)?.trim().to_owned();

    match file_content.is_empty() {
        true => Err(FileParseError::InputFileEmptyError(format!(
            "Input file was empty: {cypher_file_path}"
        ))),
        false => Ok(file_content),
    }
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

// ===============================================
// Types
// ===============================================

#[derive(Debug, Clone)]
struct CypherToken {
    // we either parse a valid string or error
    pub string_value: Result<String, FileParseError>,
    // we either parse a valid number or dont. but we also could have previous errors and thus no numeric_value to read from
    pub numeric_value: Option<Result<u32, FileParseError>>,
    pub state: NumericCoreState,
}

#[derive(Debug, Clone)]
enum FileParseError {
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
// the result stems from: we could fail to parse as the expected number type
// TODO: should we parse into u32 here or something more generic since State::new can expect any number?
//       I think the parser should not care about what we intake, just that it's a number.
fn string_to_number(word: &str) -> Result<u32, FileParseError> {
    word.chars()
        .map(|c| ((c.to_ascii_lowercase() as u32) - ('a' as u32) + 1).to_string())
        .collect::<String>()
        .parse::<u32>()
        .map_or_else(|e| Err(FileParseError::U32ParseError(e)), |value| Ok(value))
}

// number to numeric core state
// this could be a From
fn number_to_state(num: Option<u32>) -> NumericCoreState {
    NumericCoreState::new(num)
}

// ===============================================
// larger orchestrators
// ===============================================

// you COULD move the internal logic into a larger From method and convert raw text Into::into -> CypherTokens
// if word is valid
// then we can calculate numeric value -> Some(Result<u32, FileParseError>)
//     (aka some value that could fail in conversion)
// else we set numeric value to None because theres no error from the number conversion,
//     just there just isnt any value to convert in the first place
fn file_path_to_cypher_tokens(cypher_file_path: &str) -> Result<Vec<CypherToken>, FileParseError> {
    Ok(read_file_contents(cypher_file_path)?
        .split_ascii_whitespace()
        .map(|file_word| -> CypherToken {
            let word: Result<String, FileParseError> = file_word_to_string(file_word);

            let numeric_value: Option<Result<u32, FileParseError>> = match &word {
                Ok(word_value) => Some(string_to_number(word_value)),
                Err(_) => None,
            };

            let state: NumericCoreState = match &numeric_value {
                Some(Ok(u32_value)) => NumericCoreState::new(Some(*u32_value)),
                Some(Err(_)) | None => NumericCoreState::new(None::<u32>),
            };

            CypherToken {
                string_value: word,
                numeric_value: numeric_value,
                state: state,
            }
        })
        .collect_vec())
}
