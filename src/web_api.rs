use wasm_bindgen::prelude::*;

use crate::numeric_core_solver::NumericCoreSolver;

#[wasm_bindgen]
pub fn process_input_from_web(input_content: &str) -> String {
    let solver = NumericCoreSolver::new(input_content);
    let cypher_alpha = format!("{}", solver.print_cypher_alpha());
    cypher_alpha
}
