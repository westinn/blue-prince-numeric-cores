use num_traits::{FromPrimitive, Num, ToPrimitive};
use std::{
    fmt::{Debug, Display, format},
    fs,
    str::pattern::Pattern,
    u32,
};

// These are the states that we can be in while processing a Number that could be a NumericCore
// a valid NumericCore result: a whole number with 3 or less digits, >0 and <1000
// a Processable value: a whole number with 4 or more digits, aka >1000
// an Invalid result: a non-whole number, or a negative number
#[derive(Debug)]
enum NumericCoreState {
    NumericCore(NumericCoreValue),
    Processable(ProcessableValue),
    Invalid,
}

impl NumericCoreState {
    fn new<T>(input_value: T) -> Result<Self, String>
    where
        T: Num + PartialOrd + FromPrimitive + ToPrimitive + Copy + Display,
    {
        // TODO: Converted these to Result/Err, since returning None would be a faulty state
        let float_value = input_value.to_f64().ok_or(format!(
            "Unable to convert input to f64 value: {input_value}"
        ))?;
        let int_value = input_value.to_u32().ok_or(format!(
            "Unable to convert input to u32 value: {input_value}"
        ))?;

        // if non-whole number or negative, it is not valid
        if float_value.fract() != 0.0 || int_value <= 0 {
            return Ok(NumericCoreState::Invalid);
        }

        match int_value {
            1..1000 => Ok(NumericCoreState::NumericCore(NumericCoreValue(int_value))),
            1000.. => Ok(NumericCoreState::Processable(ProcessableValue(int_value))),
            _ => Err(format!(
                "Unable to use input value to create NumericCoreState: {input_value} as {int_value}"
            )),
        }
    }
}

#[derive(Debug)]
struct NumericCoreValue(u32);

impl NumericCoreValue {
    pub fn get_result(self) -> u32 {
        self.0
    }
}

#[derive(Debug)]
struct ProcessableValue(u32);

impl ProcessableValue {
    fn get_numeric_core(self) -> NumericCoreState {
        // find a value that may or may not be the final result,
        // thus we return the next State given that value
        todo!();
    }
}

/*
take cypher as matrix of strings
convert to numbers
for each number,
    is number a valid NumericCoreIteration?
        valid => create a NumericCoreIteration
        else => return that number as the core

for each NumericCoreIteration,
    solve for NumericCore and get a single iteration output
    is result a valid input to NumericCoreIteration?
        valid => create a NumericCoreIteration and continue to solve
        else => return that number as the core
*/

#[derive(Debug)]
pub struct NumericCoreSolver {
    intial_cypher_matrix: Vec<Vec<String>>,
    // numeric_cypher_matrix: Vec<Vec<u32>>
}

impl NumericCoreSolver {
    pub fn new(cypher_file_path: &str) -> Result<Self, String> {
        let file_contents: String = fs::read_to_string(cypher_file_path)
            .expect(format!("Unable to read file: {}", cypher_file_path).as_str());

        if file_contents.is_empty() {
            return Err(format!("Input file was empty: {cypher_file_path}"));
        }

        // validate that there are only ascii alphabetic characters in the file
        if let Some(faulty_word_input) = file_contents
            .trim()
            .split_ascii_whitespace()
            .find(|word| word.chars().any(|c| !c.is_ascii_alphabetic()))
        {
            return Err(format!(
                "Unable to parse word as cypher string: {faulty_word_input}"
            ));
        }

        let cypher_from_input: Vec<Vec<String>> = file_contents
            .trim()
            .lines()
            .map(|line: &str| {
                line.split_ascii_whitespace()
                    .map(|word| word.to_owned())
                    .collect()
            })
            .collect();
        Ok(NumericCoreSolver {
            intial_cypher_matrix: cypher_from_input,
        })
    }

    pub fn get_initial_cypher(&self) -> &[Vec<String>] {
        &self.intial_cypher_matrix
    }

    fn convert_to_numeric_cypher(&self) -> Vec<Vec<NumericCoreState>> {
        let initial_matrix: &[Vec<String>] = self.get_initial_cypher();
        let converted_to_numeric_cypher: Vec<Vec<u32>> = initial_matrix
            .iter()
            .map(|line: &Vec<String>| {
                line.iter()
                    .map(|word| Self::convert_word_to_number(word))
                    .collect()
            })
            .collect();

        // self.initial_cypher_matrix -> a numerical matrix
        todo!();
    }

    fn convert_word_to_number(word: &str) -> u32 {
        // will never have a non ascii alphabetical character due to solver constructor check
        let word_as_number_string: String = word
            .chars()
            .map(|c| ((c.to_ascii_lowercase() as u32) - ('a' as u32) + 1).to_string())
            .collect();
        word_as_number_string
            .parse::<u32>()
            .expect(&format!("Unable to parse word to number: {word}"))
    }

    fn solve_cypher(&self) -> Vec<Vec<u32>> {
        todo!();
        let current_cypher = self;
    }
}
