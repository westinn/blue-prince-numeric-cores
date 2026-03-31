pub mod states {
    use itertools::Itertools;
    use num_traits::{FromPrimitive, Num, ToPrimitive};
    use std::{
        fmt::{Binary, Debug, Display},
        iter::zip, ops::Not
    };

    mod binary_ops;
    use binary_ops::ops::{BinaryOp, BinaryOp::{Divide}, OP_COMBOS, NUM_OF_OPS};

    // These are the states that we can be in while processing a Number that could be a NumericCore
    // a valid NumericCore result: a whole number with 3 or less digits, >0 and <1000
    // a Processable value: a whole number with 4 or more digits, aka >1000
    // an Invalid result: a non-whole number, or a negative number

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct NumericCoreValue(u32);

    impl NumericCoreValue {
        pub fn get(&self) -> u32 {
            self.0
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ProcessableValue(u32);

    impl ProcessableValue {

        pub fn get_value(&self) -> u32 {
            self.0
        }

        fn process_value(self) {
            // get current value and digit groups
            let digit_groups: Vec<[u32; 4]> = self.value_to_digit_groups();
            // this is cheap because it's all simple enums and arrays
            let binary_op_combos = OP_COMBOS;

            // filter out any divide by 0s
            // size of ( # of op_combos * # of digit_groups )
            // a vec of instructions to calculate numeric core
            // - given the list of digit_groups (4) and the binaryops available (4)
            // - a paired up Vector of (operation to do on accumulator, number to apply op to)
            // - results in a fold function of (binary_op(accumulator, number))
            let op_digit_instr_per_digit_group: Vec<[(BinaryOp, u32); NUM_OF_OPS]> = binary_op_combos
                .into_iter()
                .cartesian_product(digit_groups) //([OP; 4], [U32; 4])
                .filter_map(|ops_digits @ (ops, digits): ([BinaryOp; NUM_OF_OPS], [u32; NUM_OF_OPS])| {
                    let zipped_op_digit: [(BinaryOp, u32); NUM_OF_OPS] = zip(ops, digits).collect_array().expect("Failed to zip array of BinaryOp and Digits.");
                    match zipped_op_digit.contains(&(Divide, 0)) {
                        true => None,
                        false => Some(zipped_op_digit),
                    }
                })
                .collect();

            // - results in a fold function of (binary_op(accumulator, number))
            let numeric_values: Vec<u32> = op_digit_instr_per_digit_group
                .iter()
                .map(|ops_for_digit_group: &[(BinaryOp, u32); 4]| {
                    ops_for_digit_group.iter().fold(
                        0,
                        |acc: u32, (curr_op, curr_number): &(BinaryOp, u32)| match curr_op {
                            Add => acc + curr_number,
                            Subtract => acc - curr_number,
                            Multiply => acc * curr_number,
                            Divide => acc / curr_number,
                        },
                    )
                })
                .collect();

            while num

            // for no_add_op_combo in no_add_op_combos {
            //     let op_combo = vec![&BINARY_OPS[..1]];
            // }
            /*
            op_combos: list[list[BinaryOp]] = [
                op_combo
                for no_add_op_combo in permutations(ops[1:])
                if (operator.truediv, 0)
                not in zip[tuple[BinaryOp, int]](
                    op_combo := ops[:1] + list(no_add_op_combo), digit_group
                )
            ]
            */
        }

        fn value_to_digit_groups(&self) -> Vec<[u32; 4]> {
            // its a u32 initially so len() is ok
            let digits_as_string: String = self.get_value().to_string();

            // we need 4 groups to calculate numeric cores
            // which means we split a list 3 times
            const GROUPS_NEEDED: usize = 4;
            const SPLIT_INDEXES_NEEDED: usize = GROUPS_NEEDED - 1;
            (0..digits_as_string.len() - 1)
                .array_combinations::<SPLIT_INDEXES_NEEDED>()
                .map(|[a, b, c]| {
                    [
                        digits_as_string[..=a].parse::<u32>().unwrap(),
                        digits_as_string[a + 1..=b].parse::<u32>().unwrap(),
                        digits_as_string[b + 1..=c].parse::<u32>().unwrap(),
                        digits_as_string[c + 1..].parse::<u32>().unwrap(),
                    ]
                })
                .collect::<Vec<[u32; 4]>>()
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum NumericCoreState {
        NumericCore(NumericCoreValue),
        Processable(ProcessableValue),
        Invalid,
    }

    impl NumericCoreState {
        pub fn new<T>(input_value: T) -> Self
        where
            T: Num + PartialOrd + FromPrimitive + ToPrimitive + Copy + Display,
        {
            if input_value <= T::zero()
                || input_value
                    .to_f64()
                    .is_some_and(|value| value.fract() != 0.0)
            {
                return NumericCoreState::Invalid;
            }

            // bounds checking for final integer value
            match input_value.to_u32() {
                Some(u32_value @ 1..1000) => {
                    NumericCoreState::NumericCore(NumericCoreValue(u32_value))
                }
                Some(u32_value @ 1000..) => {
                    NumericCoreState::Processable(ProcessableValue(u32_value))
                }
                _ => NumericCoreState::Invalid,
            }
        }

        pub fn get_numeric_core(self) -> NumericCoreState {
            match self {
                NumericCoreState::NumericCore(_) => self,
                NumericCoreState::Invalid => self,
                NumericCoreState::Processable(processable_value) => todo!(),
            }
        }
    }
}

/*
NumericCoreState::new
NumericCoreValue
ProcessableValues
*/

#[cfg(test)]
mod tests {
    use super::states::NumericCoreState;

    #[test]
    fn numeric_core_states_new() {
        // numeric core values
        let numeric_core1 = NumericCoreState::new(100);
        let numeric_core2 = NumericCoreState::new(999);
        assert!(
            matches!(numeric_core1.unwrap(), NumericCoreState::NumericCore(v) if v.get() == 100),
            "Expected NumericCore state with value 100."
        );
        assert!(
            matches!(numeric_core2.unwrap(), NumericCoreState::NumericCore(v) if v.get() == 999),
            "Expected NumericCore state with value 999."
        );

        // processable values
        let processable_value_1 = NumericCoreState::new(1000);
        let processable_value_2 = NumericCoreState::new(1500);
        assert!(
            matches!(processable_value_1.unwrap(), NumericCoreState::Processable(v) if v.get_value() == 1000),
            "Expected Processable state with value 1000."
        );
        assert!(
            matches!(processable_value_2.unwrap(), NumericCoreState::Processable(v) if v.get_value() == 1500),
            "Expected Processable state with value 1500."
        );

        // invalid inputs, fractional
        let invalid_zero_1 = NumericCoreState::new(0.0);
        let invalid_fraction_1 = NumericCoreState::new(10.2);
        let invalid_fraction_2 = NumericCoreState::new(100.2);
        let invalid_negative_1 = NumericCoreState::new(-100);
        let invalid_negative_2 = NumericCoreState::new(-10.2);
        assert!(
            matches!(invalid_zero_1.unwrap(), NumericCoreState::Invalid),
            "Expected Invalid state for zero input."
        );
        assert!(
            matches!(invalid_fraction_1.unwrap(), NumericCoreState::Invalid),
            "Expected Invalid state for fractional input."
        );
        assert!(
            matches!(invalid_fraction_2.unwrap(), NumericCoreState::Invalid),
            "Expected Invalid state for fractional input."
        );
        assert!(
            matches!(invalid_negative_1.unwrap(), NumericCoreState::Invalid),
            "Expected Invalid state for negative input."
        );
        assert!(
            matches!(invalid_negative_2.unwrap(), NumericCoreState::Invalid),
            "Expected Invalid state for negative input."
        );
    }
}

// trait ValidState {
//     fn get_numeric_core(self) -> Option<NumericCoreState>;
// }

// impl ValidState for NumericCoreValue {
//     fn get_numeric_core(self) -> Option<NumericCoreState> {
//         self
//     }
// }

// impl ProcessableValue {
//     fn get_numeric_core(self) -> Option<NumericCoreState> {
//         todo!()
//     }
// }
