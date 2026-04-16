use std::{fmt::Debug, fs, u32};

mod numeric_core_state;
mod parsers;

use numeric_core_state::states::*;
use parsers::

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
    cypher: Vec<CypherTokens>,
}

impl NumericCoreSolver {
    pub fn new(cypher_file_path: &str) -> Result<Self, String> {
        let file_contents =
            parsers::read_file_contents(cypher_file_path).map_err(|e: FileParseError| match e {
                FileParseError::Io(io_error) => {
                    eprintln!("OS Error while reading {cypher_file_path}: {}", io_error)
                }
                FileParseError::InputFileEmptyError(message) => eprintln!("{}", message),
            });

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

    // main logic

    pub fn solve_cypher(&self) -> Vec<Option<NumericCoreValue>> {
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
