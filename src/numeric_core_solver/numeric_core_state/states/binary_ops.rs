pub mod ops {
    use itertools::Itertools;
    use std::sync::LazyLock;

    use BinaryOp::{Add, Divide, Multiply, Subtract};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum BinaryOp {
        Add,
        Subtract,
        Multiply,
        Divide,
    }
    pub static NUM_OF_OPS: usize = 4;
    pub static NUM_OF_OP_COMBOS: usize = 6;

    pub static OP_COMBOS: LazyLock<[[BinaryOp; NUM_OF_OPS]; NUM_OF_OP_COMBOS]> =
        LazyLock::new(|| {
            // get binary op combos with addition always at the front
            // permutation of 3 items, selecting 3 in any order from them
            // only adding Add afterwards
            [Subtract, Multiply, Divide]
                .into_iter()
                .permutations(3)
                .map(|no_add_op_combo: Vec<BinaryOp>| -> [BinaryOp; 4] {
                    [
                        Add,
                        no_add_op_combo[1],
                        no_add_op_combo[2],
                        no_add_op_combo[3],
                    ]
                })
                .collect_array()
                .expect("Failed to generate array of BinaryOp combos.")
        });
}
