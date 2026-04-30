use std::fmt::{Debug, Display};

mod numeric_core_state;
pub(crate) mod parsers;

use itertools::Itertools;
use numeric_core_state::states::*;
use parsers::CypherToken;

use crate::numeric_core_solver::parsers::TokenNumber;

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
pub struct NumericCoreSolver<T> {
    cypher_structure: (usize, usize),
    cypher_tokens: Vec<CypherToken<T>>,
    digit_groups: Vec<Vec<DigitGroup>>,
    cypher_values: Vec<Option<NumericCoreValue>>,
    cypher_alpha: Vec<Option<char>>,
}

impl<T: TokenNumber> NumericCoreSolver<T> {
    pub fn new(input_content: &str) -> Self {
        let trimmed_input = input_content.trim();

        let cypher_structure: (usize, usize) = parsers::compute_cypher_structure(trimmed_input);

        let cypher_tokens: Vec<CypherToken<T>> = parsers::input_to_cypher_tokens(trimmed_input);

        // @TODO: this is where we would begin stacking initial potential values.
        //        rather than a Vector of DG's, it'd be a Vec<Vec<DG>>. Or wrapped in Option, potentially, in this case.
        //        This also means that the parser gives up the idea of providing the initial DigitGroup digits in the form of Vec<u32>.
        //        Parsers would stop perscribing meaning to the Tokens by turning them into values. They simply provide the valid processable inputs.
        // is is an outer vector capturing each token, and each inner vector is that tokens possible digit groups
        let digit_groups: Vec<Vec<DigitGroup>> = cypher_tokens
            .iter()
            .map(|token: &CypherToken<T>| token.into())
            .collect_vec();

        // the above digit_groups should resolve to a single NumericCoreValue during this step
        // since if there are multiple possible values, we grab the minimum one
        let cypher_values: Vec<Option<NumericCoreValue>> = digit_groups
            .iter()
            .map(
                |vec_tokens_dg: &Vec<DigitGroup>| -> Option<NumericCoreValue> {
                    vec_tokens_dg
                        .iter()
                        .filter_map(|dg: &DigitGroup| dg.into())
                        .min_by_key(|v: &NumericCoreValue| v.get_value())
                },
            )
            .collect_vec();

        let cypher_alpha: Vec<Option<char>> = cypher_values
            .iter()
            .map(
                |op_ncvalue: &Option<NumericCoreValue>| match op_ncvalue.as_ref() {
                    Some(nc_value) => char::from_u32(nc_value.get_value() + ('A' as u32) - 1),
                    None => None,
                },
            )
            .collect_vec();

        NumericCoreSolver {
            cypher_structure,
            cypher_tokens,
            digit_groups,
            cypher_values,
            cypher_alpha,
        }
    }

    // getters
    pub(crate) fn get_cypher_structure(&self) -> (usize, usize) {
        self.cypher_structure
    }

    pub(crate) fn get_cypher_tokens(&self) -> &[CypherToken<T>] {
        &self.cypher_tokens
    }

    pub(crate) fn get_digit_groups(&self) -> &[Vec<DigitGroup>] {
        &self.digit_groups
    }

    pub(crate) fn get_cypher_values(&self) -> &[Option<NumericCoreValue>] {
        &self.cypher_values
    }

    pub(crate) fn get_cypher_alpha(&self) -> &[Option<char>] {
        &self.cypher_alpha
    }

    fn print_data<V, F>(&self, data: &[V], formatter: F) -> String
    where
        F: Fn(&V) -> String,
    {
        let (xsize, _) = self.get_cypher_structure();
        data.chunks(xsize)
            .map(|row: &[V]| row.iter().map(&formatter).format(" "))
            .format("\n")
            .to_string()
    }

    pub(crate) fn print_cypher_tokens(&self) -> String {
        self.print_data(
            self.get_cypher_tokens(),
            |token: &CypherToken<T>| -> String { format!("{token:?}") },
        )
    }

    pub(crate) fn print_digit_groups(&self) -> String {
        self.print_data(self.get_digit_groups(), |vec_dg: &Vec<DigitGroup>| {
            vec_dg
                .iter()
                .map(|dg| dg.to_string())
                .format(" ")
                .to_string()
        })
    }

    pub(crate) fn print_cypher_values(&self) -> String {
        self.print_data(
            self.get_cypher_values(),
            |cypher_value: &Option<NumericCoreValue>| match cypher_value.as_ref() {
                Some(value) => format!("{value:?}"),
                None => "None".to_owned(),
            },
        )
    }

    pub(crate) fn print_cypher_alpha(&self) -> String {
        self.print_data(
            self.get_cypher_alpha(),
            |cypher_alpha: &Option<char>| match cypher_alpha.as_ref() {
                Some(value) => format!("{value}"),
                None => "None".to_owned(),
            },
        )
    }

    pub fn print_all(&self) -> String {
        format!(
            "{}",
            "\n".to_string()
                + &self.print_cypher_tokens()
                + "\n"
                + &self.print_digit_groups()
                + "\n"
                + &self.print_cypher_values()
                + "\n"
                + &self.print_cypher_alpha()
        )
    }
}

impl<T: TokenNumber> Display for NumericCoreSolver<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print_all())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = 2 + 2;
    //     assert_eq!(result, 4);
    // }
}
