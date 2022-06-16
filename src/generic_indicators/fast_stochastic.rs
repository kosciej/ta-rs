use std::fmt;

use crate::generic_indicators::{Maximum, Minimum};
use crate::{Close, High, Low, Next, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Fast stochastic oscillator.
///
/// The stochastic oscillator is a momentum indicator comparing the closing price
/// of a security to the range of its prices over a certain period of time.
///
/// # Formula
///
/// ![Fast stochastic oscillator formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/5a419041034a8044308c999f85661a08bcf91b1d)
///
/// Where:
///
/// * \%K<sub>t</sub> - value of fast stochastic oscillator
/// * C<sub>t</sub> - close price of the current period
/// * L<sub>n</sub> - lowest price for the last _n_ periods
/// * H<sub>n</sub> - highest price for the last _n_ periods
///
///
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0). Default is 14.
///
/// # Example
///
/// ```
/// use ta::generic_indicators::FastStochastic;
/// use ta::Next;
///
/// let mut stoch = FastStochastic::<5>::new();
/// assert_eq!(stoch.next(20.0), 50.0);
/// assert_eq!(stoch.next(30.0), 100.0);
/// assert_eq!(stoch.next(40.0), 100.0);
/// assert_eq!(stoch.next(35.0), 75.0);
/// assert_eq!(stoch.next(15.0), 0.0);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct FastStochastic<const N: usize = 14> {
    minimum: Minimum<N>,
    maximum: Maximum<N>,
}

impl<const N: usize> FastStochastic<N> {
    pub fn new() -> Self {
        Self {
            minimum: Minimum::new(),
            maximum: Maximum::new(),
        }
    }
}

impl<const N: usize> Period for FastStochastic<N> {
    fn period(&self) -> usize {
        N
    }
}

impl<const N: usize> Next<f64> for FastStochastic<N> {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let min = self.minimum.next(input);
        let max = self.maximum.next(input);

        if min == max {
            // When only 1 input was given, than min and max are the same,
            // therefore it makes sense to return 50
            50.0
        } else {
            (input - min) / (max - min) * 100.0
        }
    }
}

impl<T: High + Low + Close, const N: usize> Next<&T> for FastStochastic<N> {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        let highest = self.maximum.next(input.high());
        let lowest = self.minimum.next(input.low());
        let close = input.close();

        if highest == lowest {
            // To avoid division by zero, return 50.0
            50.0
        } else {
            (close - lowest) / (highest - lowest) * 100.0
        }
    }
}

impl<const N: usize> Reset for FastStochastic<N> {
    fn reset(&mut self) {
        self.minimum.reset();
        self.maximum.reset();
    }
}

impl Default for FastStochastic<14> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> fmt::Display for FastStochastic<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FAST_STOCH({})", N)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(FastStochastic);

    #[test]
    fn test_next_with_f64() {
        let mut stoch = FastStochastic::<3>::new();
        assert_eq!(stoch.next(0.0), 50.0);
        assert_eq!(stoch.next(200.0), 100.0);
        assert_eq!(stoch.next(100.0), 50.0);
        assert_eq!(stoch.next(120.0), 20.0);
        assert_eq!(stoch.next(115.0), 75.0);
    }

    #[test]
    fn test_next_with_bars() {
        let test_data = vec![
            // high, low , close, expected
            (20.0, 20.0, 20.0, 50.0), // min = 20, max = 20
            (30.0, 10.0, 25.0, 75.0), // min = 10, max = 30
            (40.0, 20.0, 16.0, 20.0), // min = 10, max = 40
            (35.0, 15.0, 19.0, 30.0), // min = 10, max = 40
            (30.0, 20.0, 25.0, 40.0), // min = 15, max = 40
            (35.0, 25.0, 30.0, 75.0), // min = 15, max = 35
        ];

        let mut stoch = FastStochastic::<3>::new();

        for (high, low, close, expected) in test_data {
            let input_bar = Bar::new().high(high).low(low).close(close);
            assert_eq!(stoch.next(&input_bar), expected);
        }
    }

    #[test]
    fn test_reset() {
        let mut indicator = FastStochastic::<10>::new();
        assert_eq!(indicator.next(10.0), 50.0);
        assert_eq!(indicator.next(210.0), 100.0);
        assert_eq!(indicator.next(10.0), 0.0);
        assert_eq!(indicator.next(60.0), 25.0);

        indicator.reset();
        assert_eq!(indicator.next(10.0), 50.0);
        assert_eq!(indicator.next(20.0), 100.0);
        assert_eq!(indicator.next(12.5), 25.0);
    }

    #[test]
    fn test_default() {
        FastStochastic::default();
    }

    #[test]
    fn test_display() {
        let indicator = FastStochastic::<21>::new();
        assert_eq!(format!("{}", indicator), "FAST_STOCH(21)");
    }
}
