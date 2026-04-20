pub mod states {
    use binary_ops::ops::{
        BinaryOp,
        BinaryOp::{Add, Divide, Multiply, Subtract},
        NUM_OF_OPS, OP_COMBOS,
    };
    use itertools::Itertools;
    use num_traits::{FromPrimitive, Num, ToPrimitive};
    use std::{
        fmt::{Debug, Display, Error},
        iter::zip,
        num::ParseIntError,
        ops::Rem,
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

    impl From<InvalidStateError> for NumericCoreState {
        fn from(_value: InvalidStateError) -> Self {
            NumericCoreState::Invalid
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

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct InvalidStateError(pub String);

    // #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    // struct _NoValidNumericCore;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TooManyPossibleValuesError;

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

        pub(crate) fn get_numeric_core(
            mut self,
        ) -> Result<Option<NumericCoreValue>, TooManyPossibleValuesError> {
            /*
            // philosophically, I think it makes more sense for a single variant of a State to process/consume only it's own state
            //   and then the greater enum to handle how, as a whole, these States should behave
            // the solver can then apply that to each state individually
            //   and this can return proper values of:
            //   an actual value, no numeric core found, or an Error of too many values
             */
            loop {
                match self {
                    NumericCoreState::Invalid => break Ok(None),
                    NumericCoreState::NumericCore(numeric_core_value) => {
                        break Ok(Some(numeric_core_value));
                    }
                    NumericCoreState::Processable(processable_value) => {
                        self = processable_value.process_value()?
                    }
                }
            }
        }

        // digit group -> single number
        // the result stems from: we could fail to parse as the expected number type
        // TODO: should we parse into u32 here or something more generic since State::new can expect any number?
        //       I think the parser should not care about what we intake, just that it's a number.
        fn digit_group_to_number(dg: &[u32]) -> Result<u32, ParseIntError> {
            let test: Result<u32, ParseIntError> = dg
                .iter()
                .map(|digit| digit.to_string())
                .collect::<String>()
                .parse::<u32>();
        }
    }

    impl NumericCoreValue {
        pub fn new(value: u32) -> Self {
            Self(value)
        }

        pub fn _get_value(&self) -> u32 {
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

        fn process_value(self) -> Result<NumericCoreState, TooManyPossibleValuesError> {
            // get current value and digit groups
            let digit_groups: Vec<[u32; 4]> = self.value_to_digit_groups();
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
                        let zipped_op_digit: [(BinaryOp, u32); NUM_OF_OPS] = zip(ops, digits).collect_array()
                            .expect("Failed to zip array of BinaryOp and Digits that would act as instructions to compute.");

                        match !zipped_op_digit.contains(&(Divide, 0)) {
                            true => Some(zipped_op_digit),
                            false => None,
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

            dbg!(
                &op_digit_processed_results
                    .iter()
                    .filter(|x| x.is_sign_positive() && x.rem(1.0) == 0.0)
                    .collect_vec()
            );

            // this is the reduction of the many numeric_results into NumericCoreStates
            // at this point, we have many potential State objects (of all kinds)
            // due to how numeric cores work, this SHOULD result in a single number after filtering for valid ones (NumericCore and Processable)
            // perhaps using an assert is valid here, givan that otherwise, this whole thing is wrong.
            let state_results: Vec<NumericCoreState> = op_digit_processed_results
                .into_iter()
                .map(|float_result| NumericCoreState::new(Some(float_result)))
                .filter(|&curr_state| !matches!(curr_state, NumericCoreState::Invalid))
                .collect();

            // can return 1 value, No value, or an Error of MANY values
            match state_results.into_iter().at_most_one() {
                Ok(Some(state)) => Ok(state),
                Ok(None) => Ok(NumericCoreState::Invalid),
                Err(recovered_iter) => {
                    eprintln!(
                        "std::ExactlyOneError: There should only be a single value after processing a ProcessableState item but found: {}.\nData: {:?}",
                        recovered_iter.len(),
                        recovered_iter
                    );
                    Err(TooManyPossibleValuesError)
                }
            }
        }

        fn value_to_digit_groups(&self) -> Vec<[u32; 4]> {
            // its a u32 initially so len() is ok
            // @TODO: I think the first parsed instance of a token is not to be combined!
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
