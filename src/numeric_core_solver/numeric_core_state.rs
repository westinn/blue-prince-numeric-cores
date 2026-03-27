mod states {
    use num_traits::{FromPrimitive, Num, ParseFloatError, Signed, ToPrimitive};
    use std::{
        fmt::{Debug, Display, format},
        num::ParseIntError,
        u32,
    };

    // These are the states that we can be in while processing a Number that could be a NumericCore
    // a valid NumericCore result: a whole number with 3 or less digits, >0 and <1000
    // a Processable value: a whole number with 4 or more digits, aka >1000
    // an Invalid result: a non-whole number, or a negative number

    #[derive(Debug)]
    struct NumericCoreValue(u32);

    #[derive(Debug)]
    struct ProcessableValue(u32);

    #[derive(Debug)]
    pub enum NumericCoreState {
        NumericCore(NumericCoreValue),
        Processable(ProcessableValue),
        Invalid,
    }

    impl NumericCoreState {
        pub fn new<T>(input_value: T) -> Result<Self, String>
        where
            T: Num + Signed + PartialOrd + FromPrimitive + ToPrimitive + Copy + Display,
        {
            let float_value = input_value.to_f64().ok_or_else(|| {
                format!("Unable to convert input to f64 value for NumericCoreState validation")
            })?;

            if float_value.fract() != 0.0 || input_value.is_negative() {
                return Ok(NumericCoreState::Invalid);
            }

            // bounds checking for final integer value
            match input_value.to_u32().ok_or_else(|| {
                format!("Unable to convert input to u32 value for NumericCoreState validation")
            }) {
                Ok(u32_value @ 1..1000) => {
                    Ok(NumericCoreState::NumericCore(NumericCoreValue(u32_value)))
                }
                Ok(u32_value @ 1000..) => {
                    Ok(NumericCoreState::Processable(ProcessableValue(u32_value)))
                }
                _ => Ok(NumericCoreState::Invalid),
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
    use super::*;

    use states::*;

    #[test]
    fn numeric_core_states_new() {
        // processable values
        let processable_value_2 = NumericCoreState::new(1000);
        let processable_value_3 = NumericCoreState::new(1500);

        // numeric core values
        let numeric_core1 = NumericCoreState::new(0);
        let numeric_core2 = NumericCoreState::new(100);

        // invalid inputs, fractional
        let invalid_fraction_1 = NumericCoreState::new(10.2);
        let invalid_fraction_3 = NumericCoreState::new(100.2);
    }

    #[test]
    #[should_panic]
    fn numeric_core_states_new_panic() {
        // invalid inputs, negative
        let invalid_negative1 = NumericCoreState::new(-1000);
        let invalid_negative2 = NumericCoreState::new(-100);

        let invalid_fraction_4 = NumericCoreState::new(-100.2);
        let invalid_fraction_2 = NumericCoreState::new(-10.2);

        // input overflow
        let f64_overflow = NumericCoreState::new(f64::MAX + 100.0);
        let u32_overflow = NumericCoreState::new(u64::MAX);
    }

    #[test]
    fn numeric_core_state_new() {
        let result = 2 + 2;
        assert_eq!(result, 4);
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
