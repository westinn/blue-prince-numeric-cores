use std::{fmt::Debug, fs, u32};

mod numeric_core_state;

use numeric_core_state::states::*;

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

#[derive(Debug, Clone)]
pub struct NumericCoreSolver {
    cypher_structure: (usize, usize),
    string_cypher: Vec<Result<String, InvalidStateError>>,
    numeric_cypher: Vec<Result<u32, InvalidStateError>>,
    state_cypher: Vec<NumericCoreState>,
}

impl NumericCoreSolver {
    pub fn new(cypher_file_path: &str) -> Result<Self, String> {
        let file_contents: String = fs::read_to_string(cypher_file_path)
            .expect(&format!("Unable to read file: {}", cypher_file_path));
        if file_contents.is_empty() {
            return Err(format!("Input file was empty: {cypher_file_path}"));
        }

        // can probably combine a lot of these steps into a single iterator with complex processing logic in a blocks, but this works for now

        // get cypher structure
        let cypher_structure: (usize, usize) = Self::compute_cypher_structure(&file_contents)?;

        // start setting the cypher matrix versions
        let string_cypher: Vec<Result<String, InvalidStateError>> =
            Self::convert_to_string_cypher(&file_contents);
        let numeric_cypher: Vec<Result<u32, InvalidStateError>> =
            Self::convert_to_numeric_cypher(&string_cypher);
        let state_cypher: Vec<NumericCoreState> = Self::convert_to_state_cypher(&numeric_cypher);

        Ok(NumericCoreSolver {
            cypher_structure,
            string_cypher,
            numeric_cypher,
            state_cypher,
        })
    }

    // static setters essentially

    fn compute_cypher_structure(file_contents: &str) -> Result<(usize, usize), String> {
        /*
        1 2 3
        1 2 3 */
        let y_cypher_rows: usize = file_contents.trim().lines().count(); // 2
        let x_cypher_longest_row: usize = file_contents // 3
            .trim()
            .lines()
            .max_by_key(|line| line.split_ascii_whitespace().count())
            .ok_or("Could not find longest row in cypher. Error occurred during initial cypher creation during parsing.")?
            .split_ascii_whitespace().count();
        // (x, y)
        Ok((x_cypher_longest_row, y_cypher_rows))
    }

    fn convert_word_to_number(word: &str) -> Result<u32, String> {
        // will never have a non ascii alphabetical character due to solver constructor check
        // thus will never be out of bounds
        let word_as_number_string: Result<u32, String> = word
            .chars()
            .map(|c| ((c.to_ascii_lowercase() as u32) - ('a' as u32) + 1).to_string())
            .collect::<String>()
            .parse::<u32>()
            .map_err(|err| {
                format!("Unable to convert word to number: {word}. Original error: {err}")
            });
        word_as_number_string
    }

    fn convert_to_string_cypher(file_contents: &str) -> Vec<Result<String, InvalidStateError>> {
        // validate that there are only ascii alphabetic characters in the file
        file_contents
            .trim()
            .split_ascii_whitespace()
            .map(
                |word: &str| match word.chars().any(|c: char| !c.is_ascii_alphabetic()) {
                    true => Ok(word.to_owned()),
                    false => Err(InvalidStateError),
                },
            )
            .collect()
    }

    fn convert_to_numeric_cypher(
        string_cypher: &[Result<String, InvalidStateError>],
    ) -> Vec<Result<u32, InvalidStateError>> {
        string_cypher
            .iter()
            .map(|word_or_invalid: &Result<String, InvalidStateError>| -> Result<u32, InvalidStateError> {
                word_or_invalid.as_ref()
                    .ok()
                    .and_then(|word_value| Self::convert_word_to_number(&word_value).ok())
                    .ok_or(InvalidStateError)
            })
            .collect()
        // @TODO: Do I exit here if initial input contains something that turns into a NumericCoreState::Invalid?
        //        I would want that to only happen after processing value further in the state machine.
        //        But maybe faulty input is fine since later values are not dependant on earlier values
        // @TODO: related to above todo, in case we do want to error out of an initial bad input state
        // let any_invalid_inputs = converted_to_numeric_cypher.iter().any(|vec_of_state| {
        //     vec_of_state
        //         .iter()
        //         .any(|state| matches!(state, NumericCoreState::Invalid))
        // });
    }

    fn convert_to_state_cypher(
        numeric_cypher: &[Result<u32, InvalidStateError>],
    ) -> Vec<NumericCoreState> {
        // parses all numbers into NumericCoreState's
        numeric_cypher
            .iter()
            .map(|num: &Result<u32, InvalidStateError>| NumericCoreState::new(num))
            .collect()
    }

    // main logic

    // @TODO: this needs to return something properly, this is heavy WIP
    //        I know that I don't want the individual state objects to handle the recursion, only to handle the next iteration.
    fn solve_cypher(&self) -> Vec<Option<NumericCoreValue>> {
        let state_cypher: &[NumericCoreState] = self.get_state_cypher();

        let numeric_cores = state_cypher
            .iter()
            .map(|current_state| {
                let current_state_value = current_state.get_numeric_core();
                match current_state_value {
                    Ok(result_value) => result_value,
                    Err(_e) => None,
                }
            })
            .collect();

        numeric_cores
    }

    // getters

    pub fn get_cypher_structure(&self) -> (usize, usize) {
        self.cypher_structure
    }

    pub fn get_string_cypher(&self) -> &Vec<Result<String, InvalidStateError>> {
        &self.string_cypher
    }

    pub fn get_numeric_cypher(&self) -> &Vec<Result<u32, InvalidStateError>> {
        &self.numeric_cypher
    }

    pub fn get_state_cypher(&self) -> &[NumericCoreState] {
        &self.state_cypher
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // functions
    /*
    NumericCoreState::new
    NumericCoreValue
    ProcessableValues

    NumericCoreSolver::new(file_path)
    ::convert_to_numeric_cypher
    ::convert_word_to_number
    ::solve_cypher
    ::get_initial_cypher
    ::get_numeric_cypher
     */

    // #[test]
    // fn it_works() {
    //     let result = 2 + 2;
    //     assert_eq!(result, 4);
    // }
}
