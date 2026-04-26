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

#[derive(Debug, Clone)]
enum TokenValue<T>
where
    T: Num + FromStr,
    <T as FromStr>::Err: Debug + Clone,
{
    Number(T),
    RomanNumeral(String),
    Word(String),
}

impl<T> TokenValue<T>
where
    T: Num + FromStr + Display + Debug,
    <T as FromStr>::Err: Debug + Clone,
{
    fn new(raw_text: &str) -> Result<Self, ParseError<T>> {
        match raw_text {
            _ if Self::is_number(raw_text) => Ok(Self::Number(
                raw_text
                    .parse::<T>()
                    .map_err(|e| ParseError::FromStrParseError(e))?,
            )),
            _ if Self::is_roman_numeral(raw_text) => Ok(Self::RomanNumeral(raw_text.to_owned())),
            _ if Self::is_word(raw_text) => Ok(Self::Word(raw_text.to_owned())),
            _ => Err(ParseError::InvalidTokenValue(format!(
                "Unable to parse raw_text into valid input: {raw_text}"
            ))),
        }
    }

    fn is_number(raw_text: &str) -> bool {
        raw_text.chars().all(|c| c.is_ascii_digit())
    }

    fn is_roman_numeral(raw_text: &str) -> bool {
        raw_text.chars().all(|c| "IVXLCDM".contains(c))
    }

    fn is_word(raw_text: &str) -> bool {
        raw_text.chars().all(|c| c.is_ascii_alphabetic())
    }

    fn to_digit_group_values(&self) -> {
        match self {
            TokenValue::Number(number) => todo!(),
            TokenValue::RomanNumeral(roman_numeral) => Self::string_to_digit_group_values(&roman_numeral),
            TokenValue::Word(word) => Self::string_to_digit_group_values(&word),
        }
    }

    fn roman_numeral_to_digit_group_values(word: &str) -> Vec<u32> {       
        word.chars().map(|c| match c {
            'I' | 'i' => 1,
            'V' | 'v' => 5,
            'X' | 'x' => 10,
            'L' | 'l' => 50,
            'C' | 'c' => 100,
            'D' | 'd' => 500,
            'M' | 'm' => 1000
        }).collect_vec()
    }

    fn string_to_digit_group_values(word: &str) -> Vec<u32> {
        word.chars()
            .map(|c| (c.to_ascii_uppercase() as u32) - ('A' as u32) + 1)
            .collect_vec()
    }

    
}

impl<T> From<&TokenValue<T>> for Option<Vec<u32>>
where
    T: Num + FromStr + Display + Debug,
    <T as FromStr>::Err: Debug + Clone,
{
    fn from(token: &TokenValue<T>) -> Self {
        match token {
            TokenValue::Number(number) => ,
            TokenValue::RomanNumeral(roman_numeral) => todo!(),
            TokenValue::Word(word) => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CypherToken<T>
where
    T: Num + FromStr + Display + Debug,
    <T as FromStr>::Err: Debug + Clone,
{
    // unprocessed input as String
    pub raw_text: String,
    // we either parse a valid string or error
    pub(crate) value: Result<TokenValue<T>, ParseError<T>>,
    // we either parse a valid number or dont
    // but we also could have previous errors and thus no numeric_value to read from
    pub(crate) initial_digit_values: Option<Vec<u32>>,
}

impl<T> CypherToken<T>
where
    T: Num + FromStr + Display + Debug,
    <T as FromStr>::Err: Debug + Clone,
{
    // TODO: could flatten these types of values so None === []
    //       though the representation of never being able to SET digits in the first place might be helpful
    pub(crate) fn get_initial_digit_values(&self) -> Option<&[u32]> {
        self.initial_digit_values.as_deref()
    }
}

impl<T> Display for CypherToken<T>
where
    T: Num + FromStr + Display + Debug,
    <T as FromStr>::Err: Debug + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?}, {:?}, {:?}",
            self.raw_text, self.value, self.initial_digit_values
        )
    }
}

#[derive(Debug, Clone)]
pub enum ParseError<T>
where
    T: Num + FromStr + Display + Debug,
    <T as FromStr>::Err: Debug + Clone,
{
    // wrap the standard library IO error
    Io(String),
    InputEmptyError(String),
    NonAsciiWord(String),
    U32ParseError(num::ParseIntError),
    FloatParseError(num::ParseFloatError),
    FromStrParseError(<T as FromStr>::Err),
    RowParseError(String),
    InvalidTokenValue(String),
}

impl<T> From<io::Error> for ParseError<T>
where
    T: Num + FromStr + Display + Debug,
    <T as FromStr>::Err: Debug + Clone,
{
    fn from(value: io::Error) -> Self {
        ParseError::Io(value.to_string())
    }
}

impl<T> From<num::ParseIntError> for ParseError<T>
where
    T: Num + FromStr + Display + Debug,
    <T as FromStr>::Err: Debug + Clone,
{
    fn from(value: num::ParseIntError) -> Self {
        ParseError::U32ParseError(value)
    }
}

impl<T> From<num::ParseFloatError> for ParseError<T>
where
    T: Num + FromStr + Display + Debug,
    <T as FromStr>::Err: Debug + Clone,
{
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
// Main Logic
// ===============================================

// ===============================================
// Inidividual value parsing
// ===============================================

// word -> valid string
// fn word_to_string<T>(word: &str) -> Result<String, ParseError<T>>
// where
//     T: Num + FromStr + Display + Debug,
//     <T as FromStr>::Err: Debug + Clone,
// {
//     match word.chars().all(|c: char| c.is_ascii_alphabetic()) {
//         true => Ok(word.to_owned()),
//         false => {
//             let err_message = format!("Unable to parse word into valid ascii string: {word}");
//             eprintln!("{}", err_message);
//             Err(ParseError::NonAsciiWord(err_message.to_owned()))
//         }
//     }
// }

// ===============================================
// larger orchestrators + related From function
// ===============================================

pub(crate) fn input_to_cypher_tokens<T>(input_content: &str) -> Vec<CypherToken<T>>
where
    T: Num + FromStr + Display + Debug,
    <T as FromStr>::Err: Debug + Clone,
{
    input_content
        .split_ascii_whitespace()
        .map(Into::into)
        .collect_vec()
}

// take an input's parsed word strings, validate them, then convert them to tokens
impl<T> From<&str> for CypherToken<T>
where
    T: Num + FromStr + Display + Debug,
    <T as FromStr>::Err: Debug + Clone,
{
    fn from(word: &str) -> Self {
        let raw_text: String = word.to_owned();
        let value: Result<TokenValue<T>, ParseError<T>> = TokenValue::new(&raw_text);

        let initial_digit_values: Option<Vec<u32>> = match &value {
            // Ok(string_value) => Some(string_to_digit_group(string_value)),
            // Err(_) => None,
            Ok(token) => DigitGroup::from(token),
            Err(e) => todo!(),
        };

        CypherToken<T> {
            raw_text,
            value,
            initial_digit_values,
        }
    }
}
