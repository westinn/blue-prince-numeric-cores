use itertools::Itertools;

fn main() {
    let OPS: [&str; 4] = ["Add", "Subtract", "Multiply", "Divide"];

    let OP_COMBOS: Vec<[String; 4]> = OPS[1..]
        .iter()
        .permutations(3)
        .map(|no_add_op_combo| -> [String; 4] {
            [
                OPS[0].to_string(),
                no_add_op_combo[0].to_string(),
                no_add_op_combo[1].to_string(),
                no_add_op_combo[2].to_string(),
            ]
        })
        .collect();

    println!("{:#?}", OP_COMBOS)
}

fn tester(OPS: [&str; 4], OP_COMBOS: Vec<[String; 4]>) {
    let digit_group: [u32; 4] = [1, 2, 3, 4];

    digit_group.
}
