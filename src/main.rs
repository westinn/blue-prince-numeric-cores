// package::module(filename)::items;
mod numeric_core_solver;

use numeric_core_solver::NumericCoreSolver;

fn main() {
    println!("Starting main!");
    let _numeric_core_solver: Result<NumericCoreSolver, String> = NumericCoreSolver::new("");
    todo!();
}
