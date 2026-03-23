use blue_prince_numeric_cores as numeric_cores;

fn main() {
    println!("Starting main!");
    let numeric_core_solver: Result<blue_prince_numeric_cores::NumericCoreSolver, String> =
        numeric_cores::NumericCoreSolver::new("");
}
