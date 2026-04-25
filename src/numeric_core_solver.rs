use std::{
    collections::btree_map::Values,
    fmt::{Debug, Display, format},
};

mod numeric_core_state;
mod parsers;

use itertools::Itertools;
use numeric_core_state::states::*;
use parsers::{CypherToken, FileParseError};

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
    cypher_tokens: Vec<CypherToken>,
    digit_groups: Vec<Option<DigitGroup>>,
    cypher: Vec<Option<NumericCoreValue>>,
}

impl NumericCoreSolver {
    pub fn new(cypher_file_path: &str) -> Result<Self, FileParseError> {
        let cypher_structure: (usize, usize) = parsers::compute_cypher_structure(cypher_file_path)?;

        let cypher_tokens: Vec<CypherToken> =
            parsers::file_path_to_cypher_tokens(cypher_file_path)?;

        let digit_groups: Vec<Option<DigitGroup>> = cypher_tokens
            .iter()
            .map(|token: &CypherToken| -> Option<DigitGroup> { token.try_into().ok() })
            .collect();

        let cypher: Vec<Option<NumericCoreValue>> = digit_groups
            .iter()
            .map(
                |op_digit_group: &Option<DigitGroup>| -> Option<NumericCoreValue> {
                    op_digit_group
                        .as_ref()
                        .and_then(|dg: &DigitGroup| dg.into())
                },
            )
            .collect_vec();

        Ok(NumericCoreSolver {
            cypher_structure,
            cypher_tokens,
            digit_groups,
            cypher,
        })
    }

    // getters
    pub(crate) fn get_cypher_structure(&self) -> (usize, usize) {
        self.cypher_structure
    }

    pub(crate) fn get_cypher_tokens(&self) -> &[CypherToken] {
        &self.cypher_tokens
    }

    pub(crate) fn get_digit_groups(&self) -> &[Option<DigitGroup>] {
        &self.digit_groups
    }

    pub(crate) fn get_cypher(&self) -> &[Option<NumericCoreValue>] {
        &self.cypher
    }

    fn print_data<T, F>(&self, data: &[T], formatter: F) -> String
    where
        F: Fn(&T) -> String,
    {
        let (_rows, columns) = self.get_cypher_structure();
        let mut result_string = String::new();

        for row in data.chunks(columns) {
            for item in row {
                result_string.push_str(&format!("{} ", formatter(item)));
            }
            result_string.push_str("\n");
        }
        result_string
    }

    pub(crate) fn print_cypher_tokens(&self) -> String {
        self.print_data(self.get_cypher_tokens(), |token: &CypherToken| -> String {
            format!("{token:?}")
        })
    }

    pub(crate) fn print_digit_groups(&self) -> String {
        self.print_data(
            self.get_digit_groups(),
            |dg: &Option<DigitGroup>| match dg.as_ref() {
                Some(values) => format!("{values:?}"),
                None => "None".to_owned(),
            },
        )
    }

    pub(crate) fn print_cypher(&self) -> String {
        self.print_data(
            self.get_cypher(),
            |cypher: &Option<NumericCoreValue>| match cypher.as_ref() {
                Some(values) => format!("{values:?}"),
                None => "None".to_owned(),
            },
        )
    }
}

impl Display for NumericCoreSolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print_cypher())
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
