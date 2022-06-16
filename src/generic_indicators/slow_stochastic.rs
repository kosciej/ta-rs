use std::fmt;

use crate::generic_indicators::{ExponentialMovingAverage, FastStochastic};
use crate::{Close, High, Low, Next, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Slow stochastic oscillator.
///
/// Basically it is a fast stochastic oscillator smoothed with exponential moving average.
///
/// # Parameters
///
/// * _stochastic_period_ - number of periods for fast stochastic (integer greater than 0). Default is 14.
/// * _ema_period_ - period for EMA (integer greater than 0). Default is 3.
///
/// # Example
///
/// ```
/// use ta::generic_indicators::SlowStochastic;
/// use ta::Next;
///
/// let mut stoch = SlowStochastic::<3, 2>::new();
/// assert_eq!(stoch.next(10.0), 50.0);
/// assert_eq!(stoch.next(50.0).round(), 83.0);
/// assert_eq!(stoch.next(50.0).round(), 94.0);
/// assert_eq!(stoch.next(30.0).round(), 31.0);
/// assert_eq!(stoch.next(55.0).round(), 77.0);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct SlowStochastic<const STOCH: usize = 14, const EMA: usize = 3> {
    fast_stochastic: FastStochastic<STOCH>,
    ema: ExponentialMovingAverage<EMA>,
}

impl<const STOCH: usize, const EMA: usize> SlowStochastic<STOCH, EMA> {
    pub fn new() -> Self {
        Self {
            fast_stochastic: FastStochastic::new(),
            ema: ExponentialMovingAverage::new(),
        }
    }
}

impl<const STOCH: usize, const EMA: usize> Next<f64> for SlowStochastic<STOCH, EMA> {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.ema.next(self.fast_stochastic.next(input))
    }
}

impl<T: High + Low + Close, const STOCH: usize, const EMA: usize> Next<&T>
    for SlowStochastic<STOCH, EMA>
{
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.ema.next(self.fast_stochastic.next(input))
    }
}

impl<const STOCH: usize, const EMA: usize> Reset for SlowStochastic<STOCH, EMA> {
    fn reset(&mut self) {
        self.fast_stochastic.reset();
        self.ema.reset();
    }
}

impl Default for SlowStochastic {
    fn default() -> Self {
        Self::new()
    }
}

impl<const STOCH: usize, const EMA: usize> fmt::Display for SlowStochastic<STOCH, EMA> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SLOW_STOCH({}, {})",
            self.fast_stochastic.period(),
            self.ema.period()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(SlowStochastic);

    #[test]
    fn test_next_with_f64() {
        let mut stoch = SlowStochastic::<3, 2>::new();
        assert_eq!(stoch.next(10.0), 50.0);
        assert_eq!(stoch.next(50.0).round(), 83.0);
        assert_eq!(stoch.next(50.0).round(), 94.0);
        assert_eq!(stoch.next(30.0).round(), 31.0);
        assert_eq!(stoch.next(55.0).round(), 77.0);
    }

    #[test]
    fn test_next_with_bars() {
        let test_data = vec![
            // high, low , close, expected
            (30.0, 10.0, 25.0, 75.0),
            (20.0, 20.0, 20.0, 58.0),
            (40.0, 20.0, 16.0, 33.0),
            (35.0, 15.0, 19.0, 22.0),
            (30.0, 20.0, 25.0, 34.0),
            (35.0, 25.0, 30.0, 61.0),
        ];

        let mut stoch = SlowStochastic::<3, 2>::new();

        for (high, low, close, expected) in test_data {
            let input_bar = Bar::new().high(high).low(low).close(close);
            assert_eq!(stoch.next(&input_bar).round(), expected);
        }
    }

    #[test]
    fn test_reset() {
        let mut stoch = SlowStochastic::<3, 2>::new();
        assert_eq!(stoch.next(10.0), 50.0);
        assert_eq!(stoch.next(50.0).round(), 83.0);
        assert_eq!(stoch.next(50.0).round(), 94.0);

        stoch.reset();
        assert_eq!(stoch.next(10.0), 50.0);
    }

    #[test]
    fn test_default() {
        SlowStochastic::default();
    }

    #[test]
    fn test_display() {
        let indicator = SlowStochastic::<10, 2>::new();
        assert_eq!(format!("{}", indicator), "SLOW_STOCH(10, 2)");
    }
}
