use super::numeric_core_state::states::NumericCoreState;
use itertools::Itertools;
use std::{fmt::Display, fs, io, num};

// ===============================================
// Types
// ===============================================

#[derive(Debug, Clone)]
pub(crate) struct CypherToken {
    // string as input from file
    pub raw_text: String,
    // we either parse a valid string or error
    pub(crate) string_value: Result<String, FileParseError>,
    // we either parse a valid number or dont
    // but we also could have previous errors and thus no numeric_value to read from
    pub(crate) numeric_value: Option<Result<u32, FileParseError>>,
    pub core_state: NumericCoreState,
}

impl Display for CypherToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?}, {:?}, {:?}, {:?})",
            self.raw_text, self.string_value, self.numeric_value, self.core_state
        )
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
// @TODO: dead code?
pub enum FileParseError {
    // wrap the standard library IO error
    Io(String),
    InputFileEmptyError(String),
    NonAsciiWord(String),
    U32ParseError(num::ParseIntError),
    RowParseError(String),
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

pub(crate) fn compute_cypher_structure(
    cypher_file_path: &str,
) -> Result<(usize, usize), FileParseError> {
    let file_contents: String = read_file_contents(cypher_file_path)?;

    let y_cypher_rows = file_contents.lines().count(); // 2
    let x_cypher_longest_row = file_contents // 3
            .lines()
            .max_by_key(|line| line.split_ascii_whitespace().count())
            .ok_or(FileParseError::RowParseError("Could not find longest row in cypher. Error occurred during initial cypher structure parsing.".to_owned()))?
            .split_ascii_whitespace()
            .count();
    // (x, y)
    Ok((x_cypher_longest_row, y_cypher_rows))
}

// ===============================================
// Main Logic
// ===============================================

// ===============================================
// Inidividual value parsing
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

// ===============================================
// larger orchestrators + related From function
// ===============================================

pub(crate) fn file_path_to_cypher_tokens(
    cypher_file_path: &str,
) -> Result<Vec<CypherToken>, FileParseError> {
    Ok(read_file_contents(cypher_file_path)?
        .split_ascii_whitespace()
        .map(Into::into)
        .collect_vec())
}

// take a file's pre-parsed word strings, validate thenm, then, convert them to tokens
impl From<&str> for CypherToken {
    fn from(file_word: &str) -> Self {
        let raw_text: String = file_word.to_owned();
        let string_value: Result<String, FileParseError> = file_word_to_string(&raw_text);
        let numeric_value: Option<Result<u32, FileParseError>> = match &string_value {
            Ok(string_value) => Some(string_to_number(string_value)),
            Err(_) => None,
        };
        let core_state: NumericCoreState = match &numeric_value {
            Some(Ok(u32_value)) => NumericCoreState::new(Some(*u32_value)),
            Some(Err(_)) | None => NumericCoreState::new(None::<u32>),
        };
        CypherToken {
            raw_text,
            string_value,
            numeric_value,
            core_state,
        }
    }
}
