use numeric_core_solver::NumericCoreSolver;

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
