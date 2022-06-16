use std::f64::INFINITY;
use std::fmt;

use crate::{Low, Next, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Returns the lowest value in a given time frame.
///
/// # Parameters
///
/// * _period_ - size of the time frame (integer greater than 0). Default value is 14.
///
/// # Example
///
/// ```
/// use ta::generic_indicators::Minimum;
/// use ta::Next;
///
/// let mut min = Minimum::<3>::new();
/// assert_eq!(min.next(10.0), 10.0);
/// assert_eq!(min.next(11.0), 10.0);
/// assert_eq!(min.next(12.0), 10.0);
/// assert_eq!(min.next(13.0), 11.0);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Minimum<const N: usize = 14> {
    min_index: usize,
    cur_index: usize,
    deque: [f64; N],
}

impl<const N: usize> Minimum<N> {
    pub fn new() -> Self {
        Self {
            min_index: 0,
            cur_index: 0,
            deque: [INFINITY; N],
        }
    }

    fn find_min_index(&self) -> usize {
        let mut min = ::std::f64::INFINITY;
        let mut index: usize = 0;

        for (i, &val) in self.deque.iter().enumerate() {
            if val < min {
                min = val;
                index = i;
            }
        }

        index
    }
}

impl<const N: usize> Period for Minimum<N> {
    fn period(&self) -> usize {
        N
    }
}

impl<const N: usize> Next<f64> for Minimum<N> {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.deque[self.cur_index] = input;

        if input < self.deque[self.min_index] {
            self.min_index = self.cur_index;
        } else if self.min_index == self.cur_index {
            self.min_index = self.find_min_index();
        }

        self.cur_index = if self.cur_index + 1 < N {
            self.cur_index + 1
        } else {
            0
        };

        self.deque[self.min_index]
    }
}

impl<T: Low, const N: usize> Next<&T> for Minimum<N> {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.low())
    }
}

impl<const N: usize> Reset for Minimum<N> {
    fn reset(&mut self) {
        for i in 0..N {
            self.deque[i] = INFINITY;
        }
    }
}

impl Default for Minimum<14> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> fmt::Display for Minimum<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MIN({})", N)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(Minimum);

    #[test]
    fn test_next() {
        let mut min = Minimum::<3>::new();

        assert_eq!(min.next(4.0), 4.0);
        assert_eq!(min.next(1.2), 1.2);
        assert_eq!(min.next(5.0), 1.2);
        assert_eq!(min.next(3.0), 1.2);
        assert_eq!(min.next(4.0), 3.0);
        assert_eq!(min.next(6.0), 3.0);
        assert_eq!(min.next(7.0), 4.0);
        assert_eq!(min.next(8.0), 6.0);
        assert_eq!(min.next(-9.0), -9.0);
        assert_eq!(min.next(0.0), -9.0);
    }

    #[test]
    fn test_next_with_bars() {
        fn bar(low: f64) -> Bar {
            Bar::new().low(low)
        }

        let mut min = Minimum::<3>::new();

        assert_eq!(min.next(&bar(4.0)), 4.0);
        assert_eq!(min.next(&bar(4.0)), 4.0);
        assert_eq!(min.next(&bar(1.2)), 1.2);
        assert_eq!(min.next(&bar(5.0)), 1.2);
    }

    #[test]
    fn test_reset() {
        let mut min = Minimum::<10>::new();

        assert_eq!(min.next(5.0), 5.0);
        assert_eq!(min.next(7.0), 5.0);

        min.reset();
        assert_eq!(min.next(8.0), 8.0);
    }

    #[test]
    fn test_default() {
        Minimum::default();
    }

    #[test]
    fn test_display() {
        let indicator = Minimum::<10>::new();
        assert_eq!(format!("{}", indicator), "MIN(10)");
    }
}
