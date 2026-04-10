/* // FinalState
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinalState {
    NumericCore(NumericCoreValue),
    Invalid,
}

impl TryFrom<NumericCoreState> for FinalState {
    type Error = NotFinalStateError;

    fn try_from(value: NumericCoreState) -> Result<Self, Self::Error> {
        match value {
            NumericCoreState::NumericCore(numeric_core_value) => {
                Ok(FinalState::NumericCore(numeric_core_value))
            }
            NumericCoreState::Invalid => Ok(FinalState::Invalid),
            _ => Err(NotFinalStateError),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotFinalStateError;
*/

/* // ValueState
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueState {
    NumericCore(NumericCoreValue),
    Processable(ProcessableValue),
}

impl ValueState {
    pub fn new<T>(input_value: Result<T, InvalidStateError>) -> Result<Self, InvalidStateError>
    where
        T: Num + PartialOrd + FromPrimitive + ToPrimitive + Copy + Display,
    {
        if input_value <= T::zero()
            || input_value
                .to_f64()
                .is_some_and(|float_value| float_value.fract() != 0.0)
        {
            return NumericCoreState::Invalid;
        }

        // bounds checking for final integer value, can simplify but being explicit here
        match input_value.to_u32() {
            Some(u32_value @ 1..1000) => {
                NumericCoreState::NumericCore(NumericCoreValue(u32_value))
            }
            Some(u32_value @ 1000..) => {
                NumericCoreState::Processable(ProcessableValue(u32_value))
            }
            Some(..=0) => NumericCoreState::Invalid,
            _ => NumericCoreState::Invalid,
        }
    }

    pub fn get_value(&self) -> u32 {
        match self {
            ValueState::NumericCore(numeric_core_value) => numeric_core_value.get_value(),
            ValueState::Processable(processable_value) => processable_value.get_value(),
        }
    }
}

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
*/
