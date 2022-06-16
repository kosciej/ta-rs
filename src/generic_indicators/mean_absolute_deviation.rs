use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Close, Next, Period, Reset};

/// Mean Absolute Deviation (MAD)
///
/// The mean absolute deviation of a data set is the average of the absolute deviations from a
/// central point. It is a summary statistic of statistical dispersion or variability.
/// In the general form, the central point can be a mean, median, mode, or the result of any other
/// measure of central tendency or any random data point related to the given data set.
/// The absolute values of the differences between the data points and their central tendency are
/// totaled and divided by the number of data points.
///
/// # Formula
///
/// MAD(_period_) = { x<sub>1</sub> - ABS(AVG(_period_)), ..., x<sub>_period_</sub> - ABS(AVG(_period_)) } / _period_
///
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0). Default is 9.
///
/// # Links
///
/// * [Mean Absolute Deviation, Wikipedia](https://en.wikipedia.org/wiki/Mean_absolute_deviation)
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct MeanAbsoluteDeviation<const N: usize = 9> {
    index: usize,
    count: usize,
    sum: f64,
    deque: [f64; N],
}

impl<const N: usize> MeanAbsoluteDeviation<N> {
    pub fn new() -> Self {
        Self {
            index: 0,
            count: 0,
            sum: 0.0,
            deque: [0.0; N],
        }
    }
}

impl<const N: usize> Period for MeanAbsoluteDeviation<N> {
    fn period(&self) -> usize {
        N
    }
}

impl<const N: usize> Next<f64> for MeanAbsoluteDeviation<N> {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.sum = if self.count < N {
            self.count = self.count + 1;
            self.sum + input
        } else {
            self.sum + input - self.deque[self.index]
        };

        self.deque[self.index] = input;
        self.index = if self.index + 1 < N {
            self.index + 1
        } else {
            0
        };

        let mean = self.sum / self.count as f64;

        let mut mad = 0.0;
        for value in &self.deque[..self.count] {
            mad += (value - mean).abs();
        }
        mad / self.count as f64
    }
}

impl<T: Close, const N: usize> Next<&T> for MeanAbsoluteDeviation<N> {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.close())
    }
}

impl<const N: usize> Reset for MeanAbsoluteDeviation<N> {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.sum = 0.0;
        for i in 0..N {
            self.deque[i] = 0.0;
        }
    }
}

impl Default for MeanAbsoluteDeviation {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> fmt::Display for MeanAbsoluteDeviation<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MAD({})", N)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(MeanAbsoluteDeviation);

    #[test]
    fn test_next() {
        let mut mad = MeanAbsoluteDeviation::<5>::new();

        assert_eq!(round(mad.next(1.5)), 0.0);
        assert_eq!(round(mad.next(4.0)), 1.25);
        assert_eq!(round(mad.next(8.0)), 2.333);
        assert_eq!(round(mad.next(4.0)), 1.813);
        assert_eq!(round(mad.next(4.0)), 1.48);
        assert_eq!(round(mad.next(1.5)), 1.48);
    }

    #[test]
    fn test_reset() {
        let mut mad = MeanAbsoluteDeviation::<5>::new();

        assert_eq!(round(mad.next(1.5)), 0.0);
        assert_eq!(round(mad.next(4.0)), 1.25);

        mad.reset();

        assert_eq!(round(mad.next(1.5)), 0.0);
        assert_eq!(round(mad.next(4.0)), 1.25);
    }

    #[test]
    fn test_default() {
        MeanAbsoluteDeviation::default();
    }

    #[test]
    fn test_display() {
        let indicator = MeanAbsoluteDeviation::<10>::new();
        assert_eq!(format!("{}", indicator), "MAD(10)");
    }
}
