use std::fmt;

use super::{ExponentialMovingAverage, TrueRange};
use crate::{Close, High, Low, Next, Period, Reset};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Average true range (ATR).
///
/// A technical analysis volatility indicator, originally developed by J. Welles Wilder.
/// The average true range is an N-day smoothed moving average of the true range values.
/// This implementation uses exponential moving average.
///
/// # Formula
///
/// ATR(period)<sub>t</sub> = EMA(period) of TR<sub>t</sub>
///
/// Where:
///
/// * _EMA(period)_ - [exponential moving average](struct.ExponentialMovingAverage.html) with smoothing period
/// * _TR<sub>t</sub>_ - [true range](struct.TrueRange.html) for period _t_
///
/// # Parameters
///
/// * _period_ - smoothing period of EMA (integer greater than 0)
///
/// # Example
///
/// ```
/// extern crate ta;
/// #[macro_use] extern crate assert_approx_eq;
///
/// use ta::{Next, DataItem};
/// use ta::generic_indicators::AverageTrueRange;
///
/// fn main() {
///     let data = vec![
///         // open, high, low, close, atr
///         (9.7   , 10.0, 9.0, 9.5  , 1.0),    // tr = high - low = 10.0 - 9.0 = 1.0
///         (9.9   , 10.4, 9.8, 10.2 , 0.95),   // tr = high - prev_close = 10.4 - 9.5 = 0.9
///         (10.1  , 10.7, 9.4, 9.7  , 1.125),  // tr = high - low = 10.7 - 9.4 = 1.3
///         (9.1   , 9.2 , 8.1, 8.4  , 1.3625), // tr = prev_close - low = 9.7 - 8.1 = 1.6
///     ];
///     let mut indicator = AverageTrueRange::<3>::new();
///
///     for (open, high, low, close, atr) in data {
///         let di = DataItem::builder()
///             .high(high)
///             .low(low)
///             .close(close)
///             .open(open)
///             .volume(1000.0)
///             .build().unwrap();
///         assert_approx_eq!(indicator.next(&di), atr);
///     }
/// }
#[doc(alias = "ATR")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct AverageTrueRange<const N: usize = 14> {
    true_range: TrueRange,
    ema: ExponentialMovingAverage<N>,
}

impl<const N: usize> AverageTrueRange<N> {
    pub fn new() -> Self {
        Self {
            true_range: TrueRange::new(),
            ema: ExponentialMovingAverage::new(),
        }
    }
}

impl<const N: usize> Period for AverageTrueRange<N> {
    fn period(&self) -> usize {
        self.ema.period()
    }
}

impl<const N: usize> Next<f64> for AverageTrueRange<N> {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.ema.next(self.true_range.next(input))
    }
}

impl<T: High + Low + Close, const N: usize> Next<&T> for AverageTrueRange<N> {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.ema.next(self.true_range.next(input))
    }
}

impl<const N: usize> Reset for AverageTrueRange<N> {
    fn reset(&mut self) {
        self.true_range.reset();
        self.ema.reset();
    }
}

impl Default for AverageTrueRange<14> {
    fn default() -> Self {
        AverageTrueRange::<14>::new()
    }
}

impl<const N: usize> fmt::Display for AverageTrueRange<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ATR({})", self.ema.period())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(AverageTrueRange);

    #[test]
    fn test_next() {
        let mut atr = AverageTrueRange::<3>::new();

        let bar1 = Bar::new().high(10).low(7.5).close(9);
        let bar2 = Bar::new().high(11).low(9).close(9.5);
        let bar3 = Bar::new().high(9).low(5).close(8);

        assert_eq!(atr.next(&bar1), 2.5);
        assert_eq!(atr.next(&bar2), 2.25);
        assert_eq!(atr.next(&bar3), 3.375);
    }

    #[test]
    fn test_reset() {
        let mut atr = AverageTrueRange::<3>::new();

        let bar1 = Bar::new().high(10).low(7.5).close(9);
        let bar2 = Bar::new().high(11).low(9).close(9.5);

        atr.next(&bar1);
        atr.next(&bar2);

        atr.reset();
        let bar3 = Bar::new().high(60).low(15).close(51);
        assert_eq!(atr.next(&bar3), 45.0);
    }

    #[test]
    fn test_default() {
        AverageTrueRange::default();
    }

    #[test]
    fn test_display() {
        let indicator = AverageTrueRange::<8>::new();
        assert_eq!(format!("{}", indicator), "ATR(8)");
    }
}
