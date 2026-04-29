mod numeric_core_solver;

use std::fs;

use numeric_core_solver::NumericCoreSolver;

fn main() {
    debug()
}

fn debug() {
    // Works
    // because its mostly web based now, this will take any string input
    let file_content = fs::read_to_string("./inputs/cypher.txt").expect("Failed to read the file!");

    let numeric_core_solver1 = NumericCoreSolver::<f64>::new(&file_content);
    println!("{numeric_core_solver1}");

    // let numeric_core_solver2 = NumericCoreSolver::new("./inputs/cypher_multiline_singleword.txt");
    // let numeric_core_solver3 = NumericCoreSolver::new("./inputs/cypher_singleline_singleword.txt");
    // let numeric_core_solver4 = NumericCoreSolver::new("./inputs/cypher_singleline_multiword.txt");

    // println!("{numeric_core_solver2}");
    // println!("{numeric_core_solver3}");
    // println!("{numeric_core_solver4}");

    // // Supposed to give some None values
    // let numeric_core_solver5 = NumericCoreSolver::new("./inputs/cypher_broken.txt");
    // let numeric_core_solver6 = NumericCoreSolver::new("./inputs/cypher_broken2.txt");

    // println!("{numeric_core_solver5}");
    // println!("{numeric_core_solver6}");

    // let tokens: String = numeric_core_solver.print_cypher_tokens();
    // let dgs: String = numeric_core_solver.print_digit_groups();
    // let cypher_values: String = numeric_core_solver.print_cypher_values();
    // let cypher_alpha: String = numeric_core_solver.print_cypher_alpha();

    // println!("{}", tokens);
    // println!("{}", dgs);
    // println!("{}", cypher_values);
    // println!("{}", cypher_alpha);
}
