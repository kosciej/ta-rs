use std::fmt;

use crate::traits::{Close, Next, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Kaufman's Efficiency Ratio (ER).
///
/// It is calculated by dividing the price change over a period by the absolute sum of the price movements that occurred to achieve that change.
/// The resulting ratio ranges between 0.0 and 1.0 with higher values representing a more efficient or trending market.
///
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0)
///
/// # Example
///
/// ```
/// use ta::generic_indicators::EfficiencyRatio;
/// use ta::Next;
///
/// let mut er = EfficiencyRatio::<4>::new();
/// assert_eq!(er.next(10.0), 1.0);
/// assert_eq!(er.next(13.0), 1.0);
/// assert_eq!(er.next(12.0), 0.5);
/// assert_eq!(er.next(13.0), 0.6);
/// assert_eq!(er.next(18.0), 0.8);
/// assert_eq!(er.next(19.0), 0.75);
/// ```

#[doc(alias = "ER")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct EfficiencyRatio<const N: usize = 14> {
    index: usize,
    count: usize,
    deque: [f64; N],
}

impl<const N: usize> EfficiencyRatio<N> {
    pub fn new() -> Self {
        Self {
            index: 0,
            count: 0,
            deque: [0.0; N],
        }
    }
}

impl<const N: usize> Period for EfficiencyRatio<N> {
    fn period(&self) -> usize {
        N
    }
}

impl<const N: usize> Next<f64> for EfficiencyRatio<N> {
    type Output = f64;

    fn next(&mut self, input: f64) -> f64 {
        let first = if self.count >= N {
            self.deque[self.index]
        } else {
            self.count += 1;
            self.deque[0]
        };
        self.deque[self.index] = input;

        self.index = if self.index + 1 < N {
            self.index + 1
        } else {
            0
        };

        let mut volatility = 0.0;
        let mut previous = first;
        for n in &self.deque[self.index..self.count] {
            volatility += (previous - n).abs();
            previous = *n;
        }
        for n in &self.deque[0..self.index] {
            volatility += (previous - n).abs();
            previous = *n;
        }

        (first - input).abs() / volatility
    }
}

impl<T: Close, const N: usize> Next<&T> for EfficiencyRatio<N> {
    type Output = f64;

    fn next(&mut self, input: &T) -> f64 {
        self.next(input.close())
    }
}

impl<const N: usize> Reset for EfficiencyRatio<N> {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        for i in 0..N {
            self.deque[i] = 0.0;
        }
    }
}

impl Default for EfficiencyRatio<14> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> fmt::Display for EfficiencyRatio<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ER({})", N)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(EfficiencyRatio);

    #[test]
    fn test_next() {
        let mut er = EfficiencyRatio::<3>::new();

        assert_eq!(round(er.next(3.0)), 1.0);
        assert_eq!(round(er.next(5.0)), 1.0);
        assert_eq!(round(er.next(2.0)), 0.2);
        assert_eq!(round(er.next(3.0)), 0.0);
        assert_eq!(round(er.next(1.0)), 0.667);
        assert_eq!(round(er.next(3.0)), 0.2);
        assert_eq!(round(er.next(4.0)), 0.2);
        assert_eq!(round(er.next(6.0)), 1.0);
    }

    #[test]
    fn test_reset() {
        let mut er = EfficiencyRatio::<3>::new();

        er.next(3.0);
        er.next(5.0);

        er.reset();

        assert_eq!(round(er.next(3.0)), 1.0);
        assert_eq!(round(er.next(5.0)), 1.0);
        assert_eq!(round(er.next(2.0)), 0.2);
        assert_eq!(round(er.next(3.0)), 0.0);
    }

    #[test]
    fn test_display() {
        let er = EfficiencyRatio::<17>::new();
        assert_eq!(format!("{}", er), "ER(17)");
    }
}
