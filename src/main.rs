mod numeric_core_solver;

use numeric_core_solver::NumericCoreSolver;

fn main() {
    debug()
}

fn debug() {
    let numeric_core_solver: NumericCoreSolver =
        NumericCoreSolver::new("./inputs/cypher_singleline_singleword.txt").unwrap();

    // let tokens: String = numeric_core_solver.print_cypher_tokens();
    // let dgs: String = numeric_core_solver.print_digit_groups();
    // let cypher_values: String = numeric_core_solver.print_cypher_values();
    // let cypher_alpha: String = numeric_core_solver.print_cypher_alpha();

    // println!("{}", tokens);
    // println!("{}", dgs);
    // println!("{}", cypher_values);
    // println!("{}", cypher_alpha);

    println!("{numeric_core_solver}");
}
