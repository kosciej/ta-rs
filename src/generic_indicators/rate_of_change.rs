use std::fmt;

use crate::traits::{Close, Next, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Rate of Change (ROC)
///
/// # Formula
///
/// ROC = (Price<sub>t</sub> - Price<sub>t-n</sub>) / Price<sub>t-n</sub> * 100
///
/// Where:
///
/// * ROC - current value of Rate of Change indicator
/// * P<sub>t</sub> - price at the moment
/// * P<sub>t-n</sub> - price _n_ periods ago
///
/// # Parameters
///
/// * _period_ - number of periods integer greater than 0
///
/// # Example
///
/// ```
/// use ta::generic_indicators::RateOfChange;
/// use ta::Next;
///
/// let mut roc = RateOfChange::<2>::new();
/// assert_eq!(roc.next(10.0), 0.0);            //  0
/// assert_eq!(roc.next(9.7).round(), -3.0);    //  (9.7 - 10) / 10  * 100 = -3
/// assert_eq!(roc.next(20.0).round(), 100.0);  //  (20 - 10)  / 10  * 100 = 100
/// assert_eq!(roc.next(20.0).round(), 106.0);  //  (20 - 9.7) / 9.7 * 100 = 106
/// ```
///
/// # Links
///
/// * [Rate of Change, Wikipedia](https://en.wikipedia.org/wiki/Momentum_(technical_analysis))
///
#[doc(alias = "ROC")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct RateOfChange<const N: usize = 9> {
    index: usize,
    count: usize,
    deque: [f64; N],
}

impl<const N: usize> RateOfChange<N> {
    pub fn new() -> Self {
        Self {
            index: 0,
            count: 0,
            deque: [0.0; N],
        }
    }
}

impl<const N: usize> Period for RateOfChange<N> {
    fn period(&self) -> usize {
        N
    }
}

impl<const N: usize> Next<f64> for RateOfChange<N> {
    type Output = f64;

    fn next(&mut self, input: f64) -> f64 {
        let previous = if self.count > N {
            self.deque[self.index]
        } else {
            self.count += 1;
            if self.count == 1 {
                input
            } else {
                self.deque[0]
            }
        };
        self.deque[self.index] = input;

        self.index = if self.index + 1 < N {
            self.index + 1
        } else {
            0
        };

        (input - previous) / previous * 100.0
    }
}

impl<T: Close, const N: usize> Next<&T> for RateOfChange<N> {
    type Output = f64;

    fn next(&mut self, input: &T) -> f64 {
        self.next(input.close())
    }
}

impl Default for RateOfChange {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> fmt::Display for RateOfChange<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ROC({})", N)
    }
}

impl<const N: usize> Reset for RateOfChange<N> {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        for i in 0..N {
            self.deque[i] = 0.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(RateOfChange);

    #[test]
    fn test_next_f64() {
        let mut roc = RateOfChange::<3>::new();

        assert_eq!(round(roc.next(10.0)), 0.0);
        assert_eq!(round(roc.next(10.4)), 4.0);
        assert_eq!(round(roc.next(10.57)), 5.7);
        assert_eq!(round(roc.next(10.8)), 8.0);
        assert_eq!(round(roc.next(10.9)), 4.808);
        assert_eq!(round(roc.next(10.0)), -5.393);
    }

    #[test]
    fn test_next_bar() {
        fn bar(close: f64) -> Bar {
            Bar::new().close(close)
        }

        let mut roc = RateOfChange::<2>::new();

        assert_eq!(round(roc.next(&bar(10.0))), 0.0);
        assert_eq!(round(roc.next(&bar(10.4))), 4.0);
        assert_eq!(round(roc.next(&bar(10.57))), 5.7);
    }

    #[test]
    fn test_reset() {
        let mut roc = RateOfChange::<3>::new();

        roc.next(12.3);
        roc.next(15.0);

        roc.reset();

        assert_eq!(round(roc.next(10.0)), 0.0);
        assert_eq!(round(roc.next(10.4)), 4.0);
        assert_eq!(round(roc.next(10.57)), 5.7);
    }
}
