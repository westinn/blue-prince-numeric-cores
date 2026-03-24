use num_traits::{FromPrimitive, Num, ToPrimitive};
use std::{
    fmt::{Debug, Display},
    fs, u32,
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
        // @TODO: Converted these to Result/Err, since returning None would be a faulty state
        let float_value = input_value.to_f64().ok_or_else(|| {
            format!("Unable to convert input to f64 value for NumericCoreState creation: {input_value} !=> f64")
        })?;
        let int_value = input_value.to_u32().ok_or_else(|| {
            format!("Unable to convert input to u32 value for NumericCoreState creation: {input_value} !=> u32")
        })?;

        // if non-whole number or negative, it is not valid
        if float_value.fract() != 0.0 || int_value <= 0 {
            return Ok(NumericCoreState::Invalid);
        }

        // bounds checking for final integer value
        match int_value {
            1..1000 => Ok(NumericCoreState::NumericCore(NumericCoreValue(int_value))),
            1000.. => Ok(NumericCoreState::Processable(ProcessableValue(int_value))),
            _ => Ok(NumericCoreState::Invalid),
        }
    }
}

// trait ValidState {
//     fn get_numeric_core(self) -> Option<NumericCoreState>;
// }

#[derive(Debug)]
struct NumericCoreValue(u32);

// impl ValidState for NumericCoreValue {
//     fn get_numeric_core(self) -> Option<NumericCoreState> {
//         self
//     }
// }

#[derive(Debug)]
struct ProcessableValue(u32);

// impl ProcessableValue {
//     fn get_numeric_core(self) -> Option<NumericCoreState> {
//         todo!()
//     }
// }

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
    numeric_cypher_matrix: Vec<Vec<NumericCoreState>>,
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
            .find(|word| word.chars().any(|c: char| !c.is_ascii_alphabetic()))
        {
            return Err(format!(
                "Unable to parse word from cypher as ascii string: {faulty_word_input}"
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

        let numeric_cypher_matrix = Self::convert_to_numeric_cypher(&cypher_from_input);

        Ok(NumericCoreSolver {
            intial_cypher_matrix: cypher_from_input,
            numeric_cypher_matrix: numeric_cypher_matrix,
        })
    }

    fn convert_to_numeric_cypher(
        initial_string_cypher: &[Vec<String>],
    ) -> Vec<Vec<NumericCoreState>> {
        let converted_to_numeric_cypher: Vec<Vec<NumericCoreState>> = initial_string_cypher
            .iter()
            .map(|line: &Vec<String>| {
                line.iter()
                    .map(|word: &String| {
                        let word_as_number: u32 = Self::convert_word_to_number(word);
                        let word_as_core_state: NumericCoreState =
                            NumericCoreState::new(word_as_number)
                                .expect(&format!("Unable to create NumericCoreState from number derived from initial cypher word: {word} as {word_as_number}"));
                        word_as_core_state
                        // @TODO: Do I exit here if initial input contains something that turns into a NumericCoreState::Invalid?
                        //        I would want that to only happen after processing value further in the state machine.
                        //        But maybe faulty input is fine since later values are not dependant on earlier values
                    })
                    .collect()
            })
            .collect();
        converted_to_numeric_cypher

        // @TODO: related to above todo, in case we do want to error out of an initial bad input state
        // let any_invalid_inputs = converted_to_numeric_cypher.iter().any(|vec_of_state| {
        //     vec_of_state
        //         .iter()
        //         .any(|state| matches!(state, NumericCoreState::Invalid))
        // });
    }

    fn convert_word_to_number(word: &str) -> u32 {
        // will never have a non ascii alphabetical character due to solver constructor check
        // thus will never be out of bounds
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

    pub fn get_initial_cypher(&self) -> &[Vec<String>] {
        &self.intial_cypher_matrix
    }

    pub fn get_numeric_cypher(&self) -> &[Vec<NumericCoreState>] {
        &self.numeric_cypher_matrix
    }
}
