//! Checked harmonic orders shared by FAST sampling and estimation.

/// A FAST harmonic order in `1..=32`.
///
/// This bound keeps bandwidth arithmetic representable and fits the
/// estimator's 32 harmonic bins. The private field prevents unchecked values.
///
/// ```compile_fail
/// use salib_samplers::HarmonicBudget;
/// let invalid = HarmonicBudget(0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u32", into = "u32"))]
pub struct HarmonicBudget(u32);

impl HarmonicBudget {
    /// Construct a harmonic order in `1..=32`.
    ///
    /// # Errors
    /// Returns [`HarmonicError::OutOfRange`] for zero or orders above 32.
    pub fn new(v: u32) -> Result<Self, HarmonicError> {
        if (1..=32).contains(&v) {
            Ok(Self(v))
        } else {
            Err(HarmonicError::OutOfRange(v))
        }
    }

    /// Return the validated harmonic order.
    #[must_use]
    pub fn get(self) -> u32 {
        self.0
    }
}

/// Failure to construct a [`HarmonicBudget`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum HarmonicError {
    /// The supplied order is outside `1..=32`.
    #[error("FAST harmonic budget must be in 1..=32, got {0}")]
    OutOfRange(u32),
}

impl TryFrom<u32> for HarmonicBudget {
    type Error = HarmonicError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<HarmonicBudget> for u32 {
    fn from(value: HarmonicBudget) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn harmonic_budget_accepts_valid_orders() {
        for value in 1..=32 {
            assert_eq!(HarmonicBudget::new(value).unwrap().get(), value);
        }
    }

    #[test]
    fn harmonic_budget_rejects_out_of_range_orders() {
        for value in [0, 33, 40, u32::MAX] {
            assert_eq!(
                HarmonicBudget::new(value),
                Err(HarmonicError::OutOfRange(value))
            );
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn harmonic_budget_deserialization_is_checked() {
        let value = HarmonicBudget::new(32).unwrap();
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(
            serde_json::from_str::<HarmonicBudget>(&json).unwrap(),
            value
        );
        for json in ["0", "33", "4294967295", "-1", "1.5", "null"] {
            assert!(serde_json::from_str::<HarmonicBudget>(json).is_err());
        }
    }
}
