mod numeric_core_solver;

use core::num;

use numeric_core_solver::NumericCoreSolver;

fn main() {
    debug()
}

fn debug() {
    let numeric_core_solver: NumericCoreSolver =
        NumericCoreSolver::new("./inputs/cypher_singleline_singleword.txt").unwrap();

    // let structure: (usize, usize) = numeric_core_solver.get_cypher_structure();
    // let string_cypher = numeric_core_solver.get_string_cypher();
    // let numeric_cypher = numeric_core_solver.get_numeric_cypher();
    // let state_cypher = numeric_core_solver.get_state_cypher();

    // println!("\n{structure:?}");
    // println!("\n{string_cypher:?}");
    // println!("\n{numeric_cypher:?}");
    // println!("\n{state_cypher:?}");

    let tokens: String = numeric_core_solver.print_cypher_tokens();
    let dgs: String = numeric_core_solver.print_digit_groups();
    let cypher: String = numeric_core_solver.print_cypher();

    println!("{}", tokens);
    println!("{}", dgs);
    println!("{}", cypher);
}
