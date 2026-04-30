use core::num;
use itertools::Itertools;
use num_traits::Num;
use std::{
    fmt::{Debug, Display},
    io,
    str::FromStr,
};

// ===============================================
// Types
// ===============================================

pub trait TokenNumber: Num + FromStr + Display + Debug + Copy {}
impl<T> TokenNumber for T where T: Num + FromStr + Display + Debug + Copy {}

#[derive(Debug, Clone)]
pub(crate) enum TokenValue<T> {
    Number(T),
    // RomanNumeral(String),
    Word(String),
}

impl<T: TokenNumber> TokenValue<T> {
    fn new(raw_text: &str) -> Result<Self, ParseError> {
        // utilities
        let is_number = |raw_text: &str| raw_text.chars().all(|c| c.is_ascii_digit());
        let is_word = |raw_text: &str| raw_text.chars().all(|c| c.is_ascii_alphabetic());

        // actual creation
        match raw_text {
            _ if is_number(raw_text) => Ok(Self::Number(raw_text.parse::<T>().map_err(|_| {
                ParseError::FromStrParseError(format!(
                    "Unable to parse text into number: {}",
                    raw_text
                ))
            })?)),
            _ if is_word(raw_text) => Ok(Self::Word(raw_text.to_owned())),
            _ => Err(ParseError::InvalidTokenValue(format!(
                "Unable to parse raw_text into valid input: {raw_text}"
            ))),
        }
        // @TODO: add this
        // let is_roman_numeral = |raw_text: &str| raw_text.chars().all(|c| "IVXLCDM".contains(c));
        // _ if Self::is_roman_numeral(raw_text) => Ok(Self::RomanNumeral(raw_text.to_owned())),
    }

    // @TODO: this needs to get every possible split of the word,
    //        filter out every set of character combos that contains a non-valid Roman numeral
    //        and then every valid combo becomes a Token that stacks together in the cypher matrix
    //        so we have not just a Vec<Tokens>, but an Vec<Vec<Tokens>>
    //        since we can have Roman numerials that have various potential initial DG values
    // fn roman_numeral_to_digit_group_values(word: &str) -> Vec<u32> {
    //     todo!();
    //     word.chars().map(|c| match c {
    //         'I' | 'i' => 1,
    //         'V' | 'v' => 5,
    //         'X' | 'x' => 10,
    //         'L' | 'l' => 50,
    //         'C' | 'c' => 100,
    //         'D' | 'd' => 500,
    //         'M' | 'm' => 1000
    //     }).collect_vec()
    // }

    // @TODO: this big section now moves to solver or state, as it determines what to do with valid TokenValues
    //        or does it make sense to keep it here as a Token object should know how to self-represent in a solver?
    //        this is complicated by the fact that a lot of processing would be handled by passing it to a ProcessableValue state more directly as we do the combinations there
    // fn number_to_digit_group_values(number: &T) -> Vec<u32> {
    //     number.to_string().chars().map(|c| c as u32).collect_vec()
    // }

    // fn string_to_digit_group_values(word: &str) -> Vec<u32> {
    //     word.chars()
    //         .map(|c| (c.to_ascii_uppercase() as u32) - ('A' as u32) + 1)
    //         .collect_vec()
    // }
}

// @TODO: this too goes to solver or state
// impl<T: TokenNumber> From<&TokenValue<T>> for Vec<Vec<u32>> {
//     fn from(token: &TokenValue<T>) -> Self {
//         match token {
//             TokenValue::Number(number) => TokenValue::number_to_digit_group_values(number),
//             //TokenValue::RomanNumeral(roman_numeral) => todo!(),
//             TokenValue::Word(word) => TokenValue::<T>::string_to_digit_group_values(word),
//         }
//     }
// }

#[derive(Debug, Clone)]
pub(crate) struct CypherToken<T> {
    // unprocessed input as String
    raw_text: String,
    // we either parse a valid string or error
    token_value: Result<TokenValue<T>, ParseError>,
    // we either parse a valid number or dont
    // but we also could have previous errors and thus no numeric_value to read from
    // digit_values: Option<Vec<u32>>,
}

impl<T: TokenNumber> CypherToken<T> {
    pub(crate) fn get_token_value(&self) -> Result<&TokenValue<T>, &ParseError> {
        self.token_value.as_ref()
    }
    // TODO: could flatten these types of values so None === []
    //       though the representation of never being able to SET digits in the first place might be helpful
    // pub(crate) fn get_initial_digit_values(&self) -> Option<&[u32]> {
    //     self.digit_values.as_deref()
    // }
}

impl<T: TokenNumber> Display for CypherToken<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?}, {:?}", // "({:?}, {:?}, {:?}",
            self.raw_text,
            self.token_value //, self.digit_values
        )
    }
}

#[derive(Debug, Clone)]
pub enum ParseError {
    Io(String),
    U32ParseError(num::ParseIntError),
    FloatParseError(num::ParseFloatError),
    FromStrParseError(String),
    InvalidTokenValue(String),
    _InputEmptyError(String),
    _RowParseError(String),
    _NonAsciiWord(String),
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

impl From<num::ParseFloatError> for ParseError {
    fn from(value: num::ParseFloatError) -> Self {
        ParseError::FloatParseError(value)
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
// larger orchestrators + related From function
// ===============================================

pub(crate) fn input_to_cypher_tokens<T: TokenNumber>(input_content: &str) -> Vec<CypherToken<T>> {
    input_content
        .split_ascii_whitespace()
        .map(Into::into)
        .collect_vec()
}

// take an input's parsed word strings, validate them, then convert them to tokens
impl<T: TokenNumber> From<&str> for CypherToken<T> {
    fn from(word: &str) -> Self {
        let raw_text: String = word.to_owned();
        let token_value: Result<TokenValue<T>, ParseError> = TokenValue::new(&raw_text);

        // @TODO: this also moves to solver
        // let digit_values: Option<Vec<u32>> = match &token_value {
        //     Ok(token_value) => Some(token_value.into()),
        //     Err(_e) => None,
        // };

        CypherToken {
            raw_text,
            token_value,
        }
    }
}
