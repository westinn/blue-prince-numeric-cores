use std::{fmt::Debug, fs, u32};

use numeric_core_state::NumericCoreState;

mod numeric_core_state;

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
struct NumericCoreSolver {
    string_cypher_matrix: Vec<Vec<String>>,
    numeric_cypher_matrix: Vec<Vec<NumericCoreState>>,
}

impl NumericCoreSolver {
    pub fn new(cypher_file_path: &str) -> Result<Self, String> {
        let file_contents: String = fs::read_to_string(cypher_file_path)
            .expect(&format!("Unable to read file: {}", cypher_file_path));

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

        let cypher_from_string_file: Vec<Vec<String>> = file_contents
            .trim()
            .lines()
            .map(|line: &str| {
                line.split_ascii_whitespace()
                    .map(|word| word.to_owned())
                    .collect()
            })
            .collect();

        let numeric_cypher_matrix = Self::convert_to_numeric_cypher(&cypher_from_string_file);

        Ok(NumericCoreSolver {
            string_cypher_matrix: cypher_from_string_file,
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
        &self.string_cypher_matrix
    }

    pub fn get_numeric_cypher(&self) -> &[Vec<NumericCoreState>] {
        &self.numeric_cypher_matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
