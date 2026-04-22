use std::fmt::{Debug, Display};

mod numeric_core_state;
mod parsers;

use itertools::Itertools;
use numeric_core_state::states::*;
use parsers::{CypherToken, FileParseError};

use crate::numeric_core_solver::numeric_core_state::states;

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
    cypher: Vec<NumericCoreState>,
}
// pub(crate) numeric_value: Option<Result<u32, FileParseError>>,
// pub core_state: NumericCoreState,

impl NumericCoreSolver {
    pub fn new(cypher_file_path: &str) -> Result<Self, FileParseError> {
        let cypher_structure: (usize, usize) = parsers::compute_cypher_structure(cypher_file_path)?;

        let cypher_tokens: Vec<CypherToken> =
            parsers::file_path_to_cypher_tokens(cypher_file_path)?;

        let digit_groups: Vec<Option<DigitGroup>> = cypher_tokens
            .iter()
            .map(|token: &CypherToken| -> Option<DigitGroup> { token.try_into().ok() })
            .collect();

        let numeric_core_states_cypher = digit_groups
            .iter()
            .map(|op_dg: &Option<DigitGroup>| -> NumericCoreState {
                // TODO?: need to keep every error but turn it into Invalid state
                let test = op_dg
                    .as_ref()
                    .map(|op_dg: &Option<DigitGroup>| match op_dg {
                        Some(dg) => NumericCoreState::try_from(dg),
                        None => NumericCoreState::new(None),
                    });
            })
            .collect_vec();

        Ok(NumericCoreSolver {
            cypher_structure,
            cypher_tokens,
            digit_groups,
            cypher: numeric_core_states_cypher,
        })
    }

    // main logic
    pub fn solve_cypher(&self) -> Vec<Option<NumericCoreValue>> {
        self.get_cypher_tokens()
            .iter()
            .map(|cypher_token: &CypherToken| -> Option<NumericCoreValue> {
                //    Result<Option<NumericCoreValue>, TooManyPossibleValues>
                match cypher_token.core_state.get_numeric_core() {
                    Ok(result_value) => result_value,
                    Err(e) => {
                        // @TODO: this shouldn't happen so do we panic or just return no value for this token?
                        eprintln!("{:?}", e);
                        None
                    }
                }
            })
            .collect()
    }

    // getters
    pub(crate) fn get_cypher_structure(&self) -> (usize, usize) {
        self.cypher_structure
    }

    pub(crate) fn get_cypher_tokens(&self) -> &[CypherToken] {
        &self.cypher_tokens
    }
}

impl Display for NumericCoreSolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (rows, columns) = self.get_cypher_structure();
        let tokens = self.get_cypher_tokens();

        let mut result_string = String::new();

        (0..rows).for_each(|row| {
            (0..columns).for_each(|col| {
                let current_token: Option<&CypherToken> = tokens.get((row * col) + col);
                result_string.push_str(&current_token.map_or_else(
                    || "None".to_owned(),
                    |token: &CypherToken| format!("{}", token),
                ));
            })
        });

        write!(f, "{}", result_string)
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
