//! Checked numeric values for public validation boundaries.

/// A finite `f64`: neither NaN nor positive or negative infinity.
///
/// The private field and checked deserialization preserve this invariant.
///
/// ```compile_fail
/// use salib_core::FiniteF64;
/// let invalid = FiniteF64(f64::NAN);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "f64", into = "f64")]
pub struct FiniteF64(f64);

impl FiniteF64 {
    /// Construct a finite value.
    ///
    /// # Errors
    /// Returns [`FiniteError::NonFinite`] for NaN or either infinity.
    pub fn new(v: f64) -> Result<Self, FiniteError> {
        if v.is_finite() {
            Ok(Self(v))
        } else {
            Err(FiniteError::NonFinite(v))
        }
    }

    /// Return the validated value.
    #[must_use]
    pub fn get(self) -> f64 {
        self.0
    }
}

/// Failure to construct a [`FiniteF64`].
#[derive(Debug, Clone, Copy, PartialEq, thiserror::Error)]
pub enum FiniteError {
    /// The supplied value is NaN or infinite.
    #[error("value must be finite, got {0}")]
    NonFinite(f64),
}

impl TryFrom<f64> for FiniteF64 {
    type Error = FiniteError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<FiniteF64> for f64 {
    fn from(value: FiniteF64) -> Self {
        value.get()
    }
}

/// A probability strictly between zero and one.
///
/// NaN, infinities, and both endpoints are rejected. Deserialization also
/// passes through the checked constructor.
///
/// ```compile_fail
/// use salib_core::Probability;
/// let invalid = Probability(1.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "f64", into = "f64")]
pub struct Probability(f64);

impl Probability {
    /// Construct a probability in the open interval `(0, 1)`.
    ///
    /// # Errors
    /// Returns [`ProbabilityError::OutOfRange`] for nonfinite values or
    /// values outside the open interval.
    pub fn new(v: f64) -> Result<Self, ProbabilityError> {
        if v > 0.0 && v < 1.0 {
            Ok(Self(v))
        } else {
            Err(ProbabilityError::OutOfRange(v))
        }
    }

    /// Return the validated probability.
    #[must_use]
    pub fn get(self) -> f64 {
        self.0
    }
}

/// Failure to construct a [`Probability`].
#[derive(Debug, Clone, Copy, PartialEq, thiserror::Error)]
pub enum ProbabilityError {
    /// The supplied value is not strictly between zero and one.
    #[error("probability must be finite and strictly between 0 and 1, got {0}")]
    OutOfRange(f64),
}

impl TryFrom<f64> for Probability {
    type Error = ProbabilityError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Probability> for f64 {
    fn from(value: Probability) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probability_accepts_open_interval() {
        for value in [
            f64::from_bits(1),
            0.1,
            0.5,
            f64::from_bits(1.0_f64.to_bits() - 1),
        ] {
            assert_eq!(
                Probability::new(value).unwrap().get().to_bits(),
                value.to_bits()
            );
        }
    }

    #[test]
    fn probability_rejects_nonfinite_and_out_of_range() {
        for value in [
            0.0,
            -0.0,
            1.0,
            -0.1,
            1.1,
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ] {
            let Err(ProbabilityError::OutOfRange(rejected)) = Probability::new(value) else {
                panic!("accepted invalid probability {value}");
            };
            assert_eq!(rejected.to_bits(), value.to_bits());
        }
    }

    #[test]
    fn probability_deserialization_is_checked() {
        let value = Probability::new(0.5).unwrap();
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(serde_json::from_str::<Probability>(&json).unwrap(), value);
        for json in ["0", "1", "-0.5", "1.5", "null", "1e400"] {
            assert!(serde_json::from_str::<Probability>(json).is_err());
        }
    }

    #[test]
    fn finite_f64_accepts_finite_values() {
        for value in [0.0, -0.0, f64::MAX, f64::MIN, f64::MIN_POSITIVE, -1.5] {
            assert_eq!(
                FiniteF64::new(value).unwrap().get().to_bits(),
                value.to_bits()
            );
        }
    }

    #[test]
    fn finite_f64_rejects_nonfinite_values() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let Err(FiniteError::NonFinite(rejected)) = FiniteF64::new(value) else {
                panic!("accepted nonfinite value {value}");
            };
            assert_eq!(rejected.to_bits(), value.to_bits());
        }
    }

    #[test]
    fn finite_f64_deserialization_is_checked() {
        let value = FiniteF64::new(-1.5).unwrap();
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(serde_json::from_str::<FiniteF64>(&json).unwrap(), value);
        for json in ["null", "1e400", "-1e400", "\"NaN\""] {
            assert!(serde_json::from_str::<FiniteF64>(json).is_err());
        }
    }
}
