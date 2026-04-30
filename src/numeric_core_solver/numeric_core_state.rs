pub mod states {
    use binary_ops::ops::{
        BinaryOp,
        BinaryOp::{Add, Divide, Multiply, Subtract},
        NUM_OF_OPS, OP_COMBOS,
    };
    use itertools::Itertools;
    use num_traits::{FromPrimitive, Num, ToPrimitive};
    use std::{
        array::{IntoIter, from_fn},
        fmt::{Debug, Display},
        num::ParseIntError,
    };

    use crate::numeric_core_solver::parsers::{CypherToken, TokenNumber, TokenValue};

    mod binary_ops;

    // ===============================================
    // Real Types
    // ===============================================

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) struct DigitGroup([u32; 4]);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub(crate) struct NumericCoreValue(u32);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub(crate) struct ProcessableValue(u32);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) enum NumericCoreState {
        NumericCore(NumericCoreValue),
        Processable(ProcessableValue),
        Invalid,
    }

    // ===============================================
    // Errors
    // ===============================================

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct InvalidStateError(pub String);
    impl From<InvalidStateError> for NumericCoreState {
        fn from(_value: InvalidStateError) -> Self {
            NumericCoreState::Invalid
        }
    }

    // ===============================================
    // Implementations
    // ===============================================
    fn number_value_to_all_digit_groups<T: TokenNumber>(value: T) -> Vec<DigitGroup> {
        // its a u32 initially so len() is ok
        let digits_as_string: String = value.to_string();

        // we need 4 groups to calculate numeric cores
        // which means we split a list 3 times
        const GROUPS_NEEDED: usize = 4;
        const SPLIT_INDEXES_NEEDED: usize = GROUPS_NEEDED - 1;
        (0..digits_as_string.len() - 1)
            .array_combinations::<SPLIT_INDEXES_NEEDED>()
            .map(|[a, b, c]| {
                DigitGroup::new(&[
                    digits_as_string[..=a].parse::<u32>().unwrap(),
                    digits_as_string[a + 1..=b].parse::<u32>().unwrap(),
                    digits_as_string[b + 1..=c].parse::<u32>().unwrap(),
                    digits_as_string[c + 1..].parse::<u32>().unwrap(),
                ])
                .unwrap()
                // this unwrap is ok because I am very manually building this instance
                // There would be an issue if I ever change the number of operators/rules of the riddle
            })
            .collect_vec()
    }

    fn word_to_digit_group(word: &str) -> Result<DigitGroup, InvalidStateError> {
        let dg_values: Vec<u32> = word
            .chars()
            .map(|c: char| (c.to_ascii_uppercase() as u32) - ('A' as u32) + 1)
            .collect_vec();

        DigitGroup::new(&dg_values)
    }

    // DigitGroup -> Option<NumericCoreValue>
    impl From<&DigitGroup> for Option<NumericCoreValue> {
        fn from(value: &DigitGroup) -> Self {
            value.process_digit_group()
        }
    }

    // basically convenience wrapper for the TryFrom below:
    impl<T: TokenNumber> From<&CypherToken<T>> for Vec<DigitGroup> {
        fn from(token: &CypherToken<T>) -> Self {
            match token.get_token_value() {
                Ok(TokenValue::Number(number)) => number_value_to_all_digit_groups(number.clone()),
                Ok(TokenValue::Word(word)) => {
                    word_to_digit_group(word).map_or_else(|_e| vec![], |v| vec![v])
                }
                Err(_e) => vec![],
            }
        }
    }

    // any slice of [u32] -> DigitGroup
    impl TryFrom<&[u32]> for DigitGroup {
        type Error = InvalidStateError;

        fn try_from(value: &[u32]) -> Result<Self, Self::Error> {
            DigitGroup::new(value)
        }
    }

    // DigitGroup -> combined digits to single u32 value
    // result because: we could fail to parse as the expected number type: u32
    // TODO: should we parse into u32 here or something more generic since State::new can expect any number?
    //       I think the parser should not care about what we intake, just that it's a number. But what does it output?
    impl TryFrom<DigitGroup> for u32 {
        type Error = ParseIntError;

        fn try_from(digit_group: DigitGroup) -> Result<Self, Self::Error> {
            digit_group
                .0
                .iter()
                .map(|digit| digit.to_string())
                .collect::<String>()
                .parse::<u32>()
        }
    }

    impl IntoIterator for DigitGroup {
        type Item = u32;
        type IntoIter = IntoIter<u32, NUM_OF_OPS>;

        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }

    impl DigitGroup {
        pub fn new(input_group: &[u32]) -> Result<Self, InvalidStateError> {
            input_group.iter().map(|&v| v).collect_array().map_or_else(
                || {
                    Err(InvalidStateError(format!(
                        "Unable to convert Vector of u32 into array of size {} during DigitGroup creation. Vector: {:?}",
                        NUM_OF_OPS, input_group
                    )))
                },
                |array: [u32; NUM_OF_OPS]| Ok(DigitGroup(array)),
            )
        }

        pub fn process_digit_group(self) -> Option<NumericCoreValue> {
            let digit_group_values: [u32; 4] = self.0;
            // 6 x [ arrays of size 4 ]
            let binary_op_combos = &OP_COMBOS;

            /*
            a vec of arrays, each of which has 4 tuples that are combined to act as instructions to calculate numeric core
            size: of ( # of op_combos * # of digit_groups )
            notes: filters out any divide by 0s
            - given the list of digit_groups (4) and the binaryops available (4)
            - a paired up Vector of (binary_operation to apply to RHS, number to act as RHS)
            - results in a fold function of (binary_op(accumulator, number))
            */
            binary_op_combos
                .into_iter()
                .filter_map(
                    // zip up all operations and the digits they will apply to
                    |ops: [BinaryOp; NUM_OF_OPS]| -> Option<[(BinaryOp, u32); NUM_OF_OPS]> {
                        // apparently this is a safer, compiler checked, alternative to zip(), just more manual
                        let zipped_op_digit: [(BinaryOp, u32); NUM_OF_OPS] =
                            from_fn(|i| (ops[i], digit_group_values[i]));
                        (!zipped_op_digit.contains(&(Divide, 0))).then_some(zipped_op_digit)
                    },
                )
                .map(
                    // use fold to apply the (operation, digit) tuple in order
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
                .filter_map(
                    // use that final number to create a new NumericCoreState and get the final core value
                    |float_result: f64| match NumericCoreState::new(Some(float_result)) {
                        NumericCoreState::Invalid => None,
                        valid_state => valid_state.get_numeric_core(),
                    },
                )
                .min()
        }
    }

    // not sure if we use this anyway to be honest
    impl<T> From<T> for NumericCoreState
    where
        T: Num + PartialOrd + FromPrimitive + ToPrimitive + Copy + Display,
    {
        fn from(value: T) -> Self {
            NumericCoreState::new(Some(value))
        }
    }

    impl NumericCoreState {
        // Create a NumericCoreState from an arbitrary number
        pub fn new<T>(input_result: Option<T>) -> Self
        where
            T: Num + PartialOrd + FromPrimitive + ToPrimitive + Copy + Display,
        {
            let invalid_value = input_result.is_none()
                || input_result.is_some_and(|value| {
                    value <= T::zero()
                        || value
                            .to_f64()
                            .is_some_and(|float_value| float_value.fract() != 0.0)
                });
            if invalid_value {
                return NumericCoreState::Invalid;
            }

            // bounds checking for final integer value, can simplify but being explicit here
            match input_result.and_then(|v| v.to_u32()) {
                Some(u32_value @ 1..1000) => {
                    NumericCoreState::NumericCore(NumericCoreValue::new(u32_value))
                }
                Some(u32_value @ 1000..) => {
                    NumericCoreState::Processable(ProcessableValue::new(u32_value))
                }
                Some(..=0) | None => NumericCoreState::Invalid,
            }
        }

        pub(crate) fn get_numeric_core(self) -> Option<NumericCoreValue> {
            match self {
                NumericCoreState::Invalid => None,
                NumericCoreState::NumericCore(numeric_core_value) => Some(numeric_core_value),
                NumericCoreState::Processable(processable_value) => {
                    processable_value.process_value()
                }
            }
        }
    }

    impl NumericCoreValue {
        pub fn new(value: u32) -> Self {
            Self(value)
        }

        pub fn get_value(&self) -> u32 {
            self.0
        }
    }

    impl ProcessableValue {
        pub fn new(value: u32) -> Self {
            Self(value)
        }

        pub fn get_value(&self) -> u32 {
            self.0
        }

        fn process_value(self) -> Option<NumericCoreValue> {
            // get current value as all possible digit groups and iterate
            let all_digit_groups: Vec<DigitGroup> =
                number_value_to_all_digit_groups(self.get_value());
            all_digit_groups
                .iter()
                .filter_map(|dg: &DigitGroup| dg.process_digit_group())
                .min_by_key(|v: &NumericCoreValue| v.get_value())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::states::NumericCoreState;

    #[test]
    fn numeric_core_states_new() {
        // numeric core values
        let numeric_core1 = NumericCoreState::new(Some(100));
        let numeric_core2 = NumericCoreState::new(Some(999));
        assert!(
            matches!(numeric_core1, NumericCoreState::NumericCore(numeric_core_value) if numeric_core_value.get_value() == 100),
            "Expected NumericCore state with value 100."
        );
        assert!(
            matches!(numeric_core2, NumericCoreState::NumericCore(numeric_core_value) if numeric_core_value.get_value() == 999),
            "Expected NumericCore state with value 999."
        );

        // processable values
        let processable_value_1 = NumericCoreState::new(Some(1000));
        let processable_value_2 = NumericCoreState::new(Some(1500));
        assert!(
            matches!(processable_value_1, NumericCoreState::Processable(processable_value) if processable_value.get_value() == 1000),
            "Expected Processable state with value 1000."
        );
        assert!(
            matches!(processable_value_2, NumericCoreState::Processable(processable_value) if processable_value.get_value() == 1500),
            "Expected Processable state with value 1500."
        );

        // invalid inputs, fractional
        let invalid_zero_1 = NumericCoreState::new(Some(0));
        let invalid_zero_2 = NumericCoreState::new(Some(0.0));
        let invalid_fraction_1 = NumericCoreState::new(Some(10.2));
        let invalid_fraction_2 = NumericCoreState::new(Some(100.2));
        let invalid_negative_1 = NumericCoreState::new(Some(-100));
        let invalid_negative_2 = NumericCoreState::new(Some(-10.2));
        assert!(
            matches!(invalid_zero_1, NumericCoreState::Invalid),
            "Expected Invalid state for zero input."
        );
        assert!(
            matches!(invalid_zero_2, NumericCoreState::Invalid),
            "Expected Invalid state for zero input."
        );
        assert!(
            matches!(invalid_fraction_1, NumericCoreState::Invalid),
            "Expected Invalid state for fractional input."
        );
        assert!(
            matches!(invalid_fraction_2, NumericCoreState::Invalid),
            "Expected Invalid state for fractional input."
        );
        assert!(
            matches!(invalid_negative_1, NumericCoreState::Invalid),
            "Expected Invalid state for negative input."
        );
        assert!(
            matches!(invalid_negative_2, NumericCoreState::Invalid),
            "Expected Invalid state for negative input."
        );
    }
}
