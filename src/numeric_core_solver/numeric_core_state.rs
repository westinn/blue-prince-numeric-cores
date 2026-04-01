pub mod states {
    use binary_ops::ops::{
        BinaryOp,
        BinaryOp::{Add, Divide, Multiply, Subtract},
        NUM_OF_OPS, OP_COMBOS,
    };
    use itertools::Itertools;
    use num_traits::{FromPrimitive, Num, ToPrimitive};
    use std::{
        fmt::{Debug, Display},
        iter::zip,
    };

    mod binary_ops;

    /*
    These are the states that we can be in while processing a Number that could be a NumericCore
    a valid NumericCore result: a whole number with 3 or less digits, >0 and <1000
    a Processable value: a whole number with 4 or more digits, aka >1000
    an Invalid result: a non-whole number, or a negative number

    there are additional traits to reflect current logical status:
    ValidState: this is a trait to reflect a state with values to consider, such as NumericCore or Processable
    ResultState: this is a trait to reflect a state that has no more steps to consider, such as NumericCore or Invalid
    */

    impl From<ValueState> for NumericCoreState {
        fn from(value: ValueState) -> Self {
            match value {
                ValueState::NumericCore(numeric_core_value) => {
                    NumericCoreState::NumericCore(numeric_core_value)
                }
                ValueState::Processable(processable_value) => {
                    NumericCoreState::Processable(processable_value)
                }
            }
        }
    }

    impl From<FinalState> for NumericCoreState {
        fn from(result: FinalState) -> Self {
            match result {
                FinalState::NumericCore(numeric_core_value) => {
                    NumericCoreState::NumericCore(numeric_core_value)
                }
                FinalState::Invalid => NumericCoreState::Invalid,
            }
        }
    }

    impl From<InvalidStateError> for NumericCoreState {
        fn from(_value: InvalidStateError) -> Self {
            NumericCoreState::Invalid
        }
    }

    impl TryFrom<NumericCoreState> for ValueState {
        type Error = InvalidStateError;

        fn try_from(value: NumericCoreState) -> Result<Self, Self::Error> {
            match value {
                NumericCoreState::NumericCore(numeric_core_value) => {
                    Ok(ValueState::NumericCore(numeric_core_value))
                }
                NumericCoreState::Processable(processable_value) => {
                    Ok(ValueState::Processable(processable_value))
                }
                NumericCoreState::Invalid => Err(InvalidStateError),
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct NumericCoreValue(u32);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ProcessableValue(u32);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum NumericCoreState {
        NumericCore(NumericCoreValue),
        Processable(ProcessableValue),
        Invalid,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ValueState {
        NumericCore(NumericCoreValue),
        Processable(ProcessableValue),
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum FinalState {
        NumericCore(NumericCoreValue),
        Invalid,
    }

    impl ValueState {
        pub fn get_value(&self) -> u32 {
            match self {
                ValueState::NumericCore(numeric_core_value) => numeric_core_value.get_value(),
                ValueState::Processable(processable_value) => processable_value.get_value(),
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct InvalidStateError;

    impl ValueState {
        pub fn new<T>(input_value: T) -> Result<Self, InvalidStateError>
        where
            T: Num + PartialOrd + FromPrimitive + ToPrimitive + Copy + Display,
        {
            if input_value <= T::zero()
                || input_value
                    .to_f64()
                    .is_some_and(|value| value.fract() != 0.0)
            {
                Err(InvalidStateError)
            } else {
                // bounds checking for final integer value
                match input_value.to_u32() {
                    Some(u32_value @ 1..1000) => {
                        Ok(ValueState::NumericCore(NumericCoreValue(u32_value)))
                    }
                    Some(u32_value @ 1000..) => {
                        Ok(ValueState::Processable(ProcessableValue(u32_value)))
                    }
                    _ => Err(InvalidStateError),
                }
            }
        }
    }

    impl NumericCoreState {
        // @TODO this is wrong since we don't want to return Value states, just the actual NumericCore state
        // we dont want Processable, and we already handle Invalid via Errors now
        pub fn get_numeric_core(self) -> Result<ValueState, InvalidStateError> {
            ValueState::try_from(self)
        }
    }

    impl NumericCoreValue {
        pub fn get_value(&self) -> u32 {
            self.0
        }
    }

    impl ProcessableValue {
        pub fn get_value(&self) -> u32 {
            self.0
        }

        fn process_value(self) -> FinalState {
            // get current value and digit groups
            let digit_groups: Vec<[u32; 4]> = self.value_to_digit_groups();
            // this is cheap because it's all simple enums and arrays
            let binary_op_combos = &OP_COMBOS;

            /*
            a vec of arrays, each of which has 4 tuples that are combined to act as instructions to calculate numeric core
            size: of ( # of op_combos * # of digit_groups )
            notes: filters out any divide by 0s
            - given the list of digit_groups (4) and the binaryops available (4)
            - a paired up Vector of (binary_operation to apply to RHS, number to act as RHS)
            - results in a fold function of (binary_op(accumulator, number))
            */
            let op_digit_numeric_core_steps: Vec<[(BinaryOp, u32); NUM_OF_OPS]> = binary_op_combos
                .into_iter()
                .cartesian_product(digit_groups) //([OP; 4], [U32; 4])
                .filter_map(
                    |(ops, digits): ([BinaryOp; NUM_OF_OPS], [u32; NUM_OF_OPS])| {
                        let zipped_op_digit: [(BinaryOp, u32); NUM_OF_OPS] = zip(ops, digits)
                            .collect_array()
                            .expect("Failed to zip array of BinaryOp and Digits.");
                        match zipped_op_digit.contains(&(Divide, 0)) {
                            true => None,
                            false => Some(zipped_op_digit),
                        }
                    },
                )
                .collect();

            // [(BinaryOp, u32); 4]

            let op_digit_processed_results: Vec<f64> = op_digit_numeric_core_steps
                .into_iter()
                .map(
                    |ops_for_digit_group: [(BinaryOp, u32); NUM_OF_OPS]| -> f64 {
                        ops_for_digit_group.into_iter().fold(
                            0.0,
                            |acc: f64, (curr_op, curr_number): (BinaryOp, u32)| {
                                let curr_number_as_f64: f64 = f64::from(curr_number);
                                match curr_op {
                                    Add => acc + curr_number_as_f64,
                                    Subtract => acc - curr_number_as_f64,
                                    Multiply => acc * curr_number_as_f64,
                                    Divide => acc / curr_number_as_f64,
                                }
                            },
                        )
                    },
                )
                .collect();

            /*
            @TODO:
            now that we have a list of all possible operations on all possible digit groups,
            is there where I turn them all into NumericCoreStates and use their status to filter?
            if I just filter here, then if the criteria for numericcores changes, refactoring is a pain
            if I convert to numeric cores here, I delegate the state properly. then I filter off the most valid numericcorestate objects
            */
            let state_results = op_digit_processed_results
                .into_iter()
                .map(|numeric_result: f64| -> NumericCoreState {
                    NumericCoreState::new(numeric_result).get_numeric_core()
                })
                .filter(|state| match state {
                    NumericCoreState::NumericCore(_) => true,
                    NumericCoreState::Invalid => false,
                });

            todo!()
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
