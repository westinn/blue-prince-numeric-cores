pub mod states {
    use num_traits::{FromPrimitive, Num, ToPrimitive};
    use std::fmt::{Debug, Display};

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
        pub fn get(&self) -> u32 {
            self.0
        }

        pub fn get_numeric_core(&self) -> NumericCoreState {
            todo!();
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum NumericCoreState {
        NumericCore(NumericCoreValue),
        Processable(ProcessableValue),
        Invalid,
    }

    impl NumericCoreState {
        pub fn new<T>(input_value: T) -> Result<Self, String>
        where
            T: Num + PartialOrd + FromPrimitive + ToPrimitive + Copy + Display,
        {
            if input_value <= T::zero()
                || input_value
                    .to_f64()
                    .is_some_and(|value| value.fract() != 0.0)
            {
                return Ok(NumericCoreState::Invalid);
            }

            // bounds checking for final integer value
            match input_value.to_u32() {
                Some(u32_value @ 1..1000) => {
                    Ok(NumericCoreState::NumericCore(NumericCoreValue(u32_value)))
                }
                Some(u32_value @ 1000..) => {
                    Ok(NumericCoreState::Processable(ProcessableValue(u32_value)))
                }
                _ => Ok(NumericCoreState::Invalid),
            }
        }

        pub fn get_numeric_core(&self) -> NumericCoreState {
            match self {
                NumericCoreState::Processable(processable_value) => {
                    processable_value.get_numeric_core()
                }
                NumericCoreState::NumericCore(numeric_core_value) => {
                    NumericCoreState::NumericCore(*numeric_core_value)
                }
                NumericCoreState::Invalid => NumericCoreState::Invalid,
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
            matches!(processable_value_1.unwrap(), NumericCoreState::Processable(v) if v.get() == 1000),
            "Expected Processable state with value 1000."
        );
        assert!(
            matches!(processable_value_2.unwrap(), NumericCoreState::Processable(v) if v.get() == 1500),
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
