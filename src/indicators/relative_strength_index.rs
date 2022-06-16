use std::fmt;

use crate::indicators::ExponentialMovingAverage as Ema;
use crate::{Close, Next, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The relative strength index (RSI).
///
/// It is a momentum oscillator,
/// that compares the magnitude of recent gains
/// and losses over a specified time period to measure speed and change of price
/// movements of a security. It is primarily used to attempt to identify
/// overbought or oversold conditions in the trading of an asset.
///
/// The oscillator returns output in the range of 0..100.
///
/// ![RSI](https://upload.wikimedia.org/wikipedia/commons/6/67/RSIwiki.gif)
///
/// # Formula
///
/// RSI<sub>t</sub> = EMA<sub>Ut</sub> * 100 / (EMA<sub>Ut</sub> + EMA<sub>Dt</sub>)
///
/// Where:
///
/// * RSI<sub>t</sub> - value of RSI indicator in a moment of time _t_
/// * EMA<sub>Ut</sub> - value of [EMA](struct.ExponentialMovingAverage.html) of up periods in a moment of time _t_
/// * EMA<sub>Dt</sub> - value of [EMA](struct.ExponentialMovingAverage.html) of down periods in a moment of time _t_
///
/// If current period has value higher than previous period, than:
///
/// U = p<sub>t</sub> - p<sub>t-1</sub>
///
/// D = 0
///
/// Otherwise:
///
/// U = 0
///
/// D = p<sub>t-1</sub> - p<sub>t</sub>
///
/// Where:
///
/// * U = up period value
/// * D = down period value
/// * p<sub>t</sub> - input value in a moment of time _t_
/// * p<sub>t-1</sub> - input value in a moment of time _t-1_
///
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0). Default value is 14.
///
/// # Example
///
/// ```
/// use ta::indicators::RelativeStrengthIndex;
/// use ta::Next;
///
/// let mut rsi = RelativeStrengthIndex::<3>::new();
/// assert_eq!(rsi.next(10.0), 50.0);
/// assert_eq!(rsi.next(10.5).round(), 86.0);
/// assert_eq!(rsi.next(10.0).round(), 35.0);
/// assert_eq!(rsi.next(9.5).round(), 16.0);
/// ```
///
/// # Links
/// * [Relative strength index (Wikipedia)](https://en.wikipedia.org/wiki/Relative_strength_index)
/// * [RSI (Investopedia)](http://www.investopedia.com/terms/r/rsi.asp)
///
#[doc(alias = "RSI")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct RelativeStrengthIndex<const N: usize = 14> {
    up_ema_indicator: Ema<N>,
    down_ema_indicator: Ema<N>,
    prev_val: f64,
    is_new: bool,
}

impl<const N: usize> RelativeStrengthIndex<N> {
    pub fn new() -> Self {
        Self {
            up_ema_indicator: Ema::new(),
            down_ema_indicator: Ema::new(),
            prev_val: 0.0,
            is_new: true,
        }
    }
}

impl<const N: usize> Period for RelativeStrengthIndex<N> {
    fn period(&self) -> usize {
        N
    }
}

impl<const N: usize> Next<f64> for RelativeStrengthIndex<N> {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let mut up = 0.0;
        let mut down = 0.0;

        if self.is_new {
            self.is_new = false;
            // Initialize with some small seed numbers to avoid division by zero
            up = 0.1;
            down = 0.1;
        } else {
            if input > self.prev_val {
                up = input - self.prev_val;
            } else {
                down = self.prev_val - input;
            }
        }

        self.prev_val = input;
        let up_ema = self.up_ema_indicator.next(up);
        let down_ema = self.down_ema_indicator.next(down);
        100.0 * up_ema / (up_ema + down_ema)
    }
}

impl<T: Close, const N: usize> Next<&T> for RelativeStrengthIndex<N> {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.close())
    }
}

impl<const N: usize> Reset for RelativeStrengthIndex<N> {
    fn reset(&mut self) {
        self.is_new = true;
        self.prev_val = 0.0;
        self.up_ema_indicator.reset();
        self.down_ema_indicator.reset();
    }
}

impl Default for RelativeStrengthIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> fmt::Display for RelativeStrengthIndex<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RSI({})", N)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(RelativeStrengthIndex);

    #[test]
    fn test_next() {
        let mut rsi = RelativeStrengthIndex::<3>::new();
        assert_eq!(rsi.next(10.0), 50.0);
        assert_eq!(rsi.next(10.5).round(), 86.0);
        assert_eq!(rsi.next(10.0).round(), 35.0);
        assert_eq!(rsi.next(9.5).round(), 16.0);
    }

    #[test]
    fn test_reset() {
        let mut rsi = RelativeStrengthIndex::<3>::new();
        assert_eq!(rsi.next(10.0), 50.0);
        assert_eq!(rsi.next(10.5).round(), 86.0);

        rsi.reset();
        assert_eq!(rsi.next(10.0).round(), 50.0);
        assert_eq!(rsi.next(10.5).round(), 86.0);
    }

    #[test]
    fn test_default() {
        RelativeStrengthIndex::default();
    }

    #[test]
    fn test_display() {
        let rsi = RelativeStrengthIndex::<16>::new();
        assert_eq!(format!("{}", rsi), "RSI(16)");
    }
}
