use itertools::Itertools;
use std::{fmt::Display, fs, io, num};

/*
==========================================
Basic parser to Solver logical order:

=====================
=> Parser
- file contents read as 1 String
    - file contents split into individual Strings, aka raw_text
    - each String is parsed into valid words, aka string_value
    - each valid word is parsed as individual numbers per letter, aka an initial_digit_group
- those 3 are packaged into a single CypherToken

=====================
notes:
- each CypherToken's initial_digit_group has to be processed as is
    - normally a State's takes in a single "value: u32"
    - State figures out what variant of State that number is
    - if Processable, that number is then split into MANY digit groups
    - and each digit group then gets processed, and returns a possible State

// possibilities?
- I could add in a new NumericCoreState variant that is an InitialDG to process
    - but that would need refactors elsewhere to create proper matches? I think?
- I could rewrite NumericCoreState to take in a Digit Group
    - but that doesn't match the intent since a State's value then splits up.
    - but it does match the flow of data.
        - we have digit groups, make a State that holds a digit group, and can process it as a number
        - but if we have digit groups, how do we pass it fractional values and check if that's a valid entry?
        - I don't think we can, so I think the Digit Group part lives in the variant
- I could write a new Object that is a single Digit Group that can do it's own processing
    - but then I'm not reusing a lot of the code elsewhere
    - but maybe I write it and then see what is actuall reuse and what is "new"

=====================
=> Solver
- each CypherToken's initial_digit_group must be
    - NumericCoreState::new() takes in a Digit Group and sets the current state
    - but each State also needs to hold the DG that created itself, so it knows how to process it's current iteration

==========================================
*/

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
    pub(crate) initial_digit_group: Option<Vec<u32>>,
}

impl Display for CypherToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?}, {:?}, {:?}",
            self.raw_text, self.string_value, self.initial_digit_group
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
    match word.chars().all(|c: char| c.is_ascii_alphabetic()) {
        true => Ok(word.to_owned()),
        false => {
            let err_message =
                format!("Unable to parse word in file into valid ascii string: {word}.");
            eprintln!("{}", err_message);
            Err(FileParseError::NonAsciiWord(err_message.to_owned()))
        }
    }
}

// string -> digit group of u32
fn string_to_digit_group(word: &str) -> Vec<u32> {
    word.chars()
        .map(|c| (c.to_ascii_lowercase() as u32) - ('a' as u32) + 1)
        .collect_vec()
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

        let initial_digit_group: Option<Vec<u32>> = match &string_value {
            Ok(string_value) => Some(string_to_digit_group(string_value)),
            Err(_) => None,
        };

        // let numeric_value: Option<Result<u32, FileParseError>> = match &digit_group {
        //     Some(digit_group) => Some(digit_group_to_number(digit_group)),
        //     None => None,
        // };

        // let core_state: NumericCoreState = match &numeric_value {
        //     Some(Ok(u32_value)) => NumericCoreState::new(Some(*u32_value)),
        //     Some(Err(_)) | None => NumericCoreState::new(None::<u32>),
        // };

        CypherToken {
            raw_text,
            string_value,
            initial_digit_group,
            // numeric_value,
            // core_state,
        }
    }
}
