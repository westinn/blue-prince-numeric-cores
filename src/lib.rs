use num_traits::{FromPrimitive, Num, ToPrimitive};
use std::{fmt::Debug, fs};

static ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

// These are the states that we can be in while processing a Number that could be a NumericCore
// a valid NumericCore result: a whole number with 3 or less digits, >0 and <1000
// a Processable value: a whole number with 4 or more digits, aka >1000
// an Invalid result: a non-whole number, or a negative number
#[derive(Debug)]
enum NumericCoreState {
    NumericCoreResult(NumericCore),
    Processable(IterationValue),
    Invalid,
}

impl NumericCoreState {
    fn new<T>(input_value: T) -> Option<Self>
    where
        T: Num + PartialOrd + FromPrimitive + ToPrimitive + Copy,
    {
        let float_cast_value = input_value.to_f64()?;
        let result_value = input_value.to_u32()?;
        
        if result_value >= 1000 && input_value.to_f64()?.fract() == 0.0 {
            Some(Self {
                value: result_value,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct NumericCore {
    result: u32,
}

impl NumericCore {
    fn valid_numeric_core () {

    }
}

// This is the value that can continue to be processed to find a NumericCore
#[derive(Debug)]
struct IterationValue {
    value: u32,
}

impl IterationValue {
    fn get_numeric_core_iteration(self) -> NumericCoreState {
        let current_value = self.value;
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
        //
        let file_contents: String = fs::read_to_string(cypher_file_path)
            .expect(format!("Unable to read file: {}", cypher_file_path).as_str());

        if file_contents.is_empty() {
            return Err(format!("Input file was empty: {}", cypher_file_path));
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
            intial_cypher_matrix: cypher_from_input
        })
    }

    pub fn get_initial_cypher(&self) -> &[Vec<String>] {
        &self.intial_cypher_matrix
    }

    fn convert_to_numeric_cypher(&self) -> Vec<Vec<NumericCoreState>> {
        let initial_matrix: &[Vec<String>] = self.get_initial_cypher();
        let test: Vec<Vec<u32>> = initial_matrix
            .iter()
            .map(|line: &Vec<String>| 
                line
                .iter()
                .map(|word| Self::convert_word_to_number(word))
                .collect())
            .collect();

        // self.initial_cypher_matrix -> a numerical matrix
        todo!();
    }

    fn solve_cypher(&self) -> Vec<Vec<u32>> {
        let current_cypher = self.
        todo!();
    }

    fn convert_word_to_number(word: &str) -> u32 {

        todo!();
    }
}

/*
// Auto-generated tests by Cargo
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_numeric_core_solver() {
        let cypher = NumericCoreSolver::new("./inputs/cypher.txt");
        let cypher_multiline_singleword =
            NumericCoreSolver::new("./inputs/cypher_multiline_singleword.txt");
        let cypher_singleline_multiword =
            NumericCoreSolver::new("./inputs/cypher_singleline_multiword.txt");
        let cypher_singleline_singleword =
            NumericCoreSolver::new("./inputs/cypher_singleline_singleword.txt");

        println!("{:?}", cypher);
        println!("{:?}", cypher_multiline_singleword);
        println!("{:?}", cypher_singleline_multiword);
        println!("{:?}", cypher_singleline_singleword);
    }
}
