use std::fmt;

use crate::{Close, High, Low, Next, Period, Reset, Volume};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Money Flow Index (MFI).
///
/// The MFI is an volume and price based oscillator which gives moneyflow over n periods.
/// MFI is used to measure buying and selling pressure.
/// MFI is also known as volume-weighted RSI.
///
/// # Formula
///
/// Typical Price(TP) = (High + Low + Close)/3
///
/// Money Flow(MF) = Typical Price x Volume
///
/// MF is positive when currennt TP is greater that previous period TP and
/// negative when current TP is less than preivous TP.
///
/// Positive money flow (PMF)- calculated by adding the money flow of all the days RMF is positive.
///
/// Negative money flow (NMF)- calculated by adding the money flow of all the days RMF is negative.
///
/// Money Flow Index(MFI) = PMF / (PMF + NMF) * 100
///
///
/// # Parameters
///
/// * _period_ - number of periods, integer greater than 0
///
/// # Example
///
/// ```
/// use ta::generic_indicators::MoneyFlowIndex;
/// use ta::{Next, DataItem};
///
/// let mut mfi = MoneyFlowIndex::<3>::new();
/// let di = DataItem::builder()
///             .high(3.0)
///             .low(1.0)
///             .close(2.0)
///             .open(1.5)
///             .volume(1000.0)
///             .build().unwrap();
/// mfi.next(&di);
///
/// ```
/// # Links
/// * [Money Flow Index, Wikipedia](https://en.wikipedia.org/wiki/Money_flow_index)
/// * [Money Flow Index, stockcharts](https://stockcharts.com/school/doku.php?id=chart_school:technical_indicators:money_flow_index_mfi)

#[doc(alias = "MFI")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct MoneyFlowIndex<const N: usize = 14> {
    index: usize,
    count: usize,
    previous_typical_price: f64,
    total_positive_money_flow: f64,
    total_negative_money_flow: f64,
    deque: [f64; N],
}

impl<const N: usize> MoneyFlowIndex<N> {
    pub fn new() -> Self {
        Self {
            index: 0,
            count: 0,
            previous_typical_price: 0.0,
            total_positive_money_flow: 0.0,
            total_negative_money_flow: 0.0,
            deque: [0.0; N],
        }
    }
}

impl<const N: usize> Period for MoneyFlowIndex<N> {
    fn period(&self) -> usize {
        N
    }
}

impl<T: High + Low + Close + Volume, const N: usize> Next<&T> for MoneyFlowIndex<N> {
    type Output = f64;

    fn next(&mut self, input: &T) -> f64 {
        let tp = (input.close() + input.high() + input.low()) / 3.0;

        self.index = if self.index + 1 < N {
            self.index + 1
        } else {
            0
        };

        if self.count < N {
            self.count = self.count + 1;
            if self.count == 1 {
                self.previous_typical_price = tp;
                return 50.0;
            }
        } else {
            let popped = self.deque[self.index];
            if popped.is_sign_positive() {
                self.total_positive_money_flow -= popped;
            } else {
                self.total_negative_money_flow += popped;
            }
        }

        if tp > self.previous_typical_price {
            let raw_money_flow = tp * input.volume();
            self.total_positive_money_flow += raw_money_flow;
            self.deque[self.index] = raw_money_flow;
        } else if tp < self.previous_typical_price {
            let raw_money_flow = tp * input.volume();
            self.total_negative_money_flow += raw_money_flow;
            self.deque[self.index] = -raw_money_flow;
        } else {
            self.deque[self.index] = 0.0;
        }
        self.previous_typical_price = tp;

        self.total_positive_money_flow
            / (self.total_positive_money_flow + self.total_negative_money_flow)
            * 100.0
    }
}

impl Default for MoneyFlowIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> fmt::Display for MoneyFlowIndex<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MFI({})", N)
    }
}

impl<const N: usize> Reset for MoneyFlowIndex<N> {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.previous_typical_price = 0.0;
        self.total_positive_money_flow = 0.0;
        self.total_negative_money_flow = 0.0;
        for i in 0..N {
            self.deque[i] = 0.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    #[test]
    fn test_next_bar() {
        let mut mfi = MoneyFlowIndex::<3>::new();

        let bar1 = Bar::new().high(3).low(1).close(2).volume(500.0);
        assert_eq!(round(mfi.next(&bar1)), 50.0);

        let bar2 = Bar::new().high(2.3).low(2.0).close(2.3).volume(1000.0);
        assert_eq!(round(mfi.next(&bar2)), 100.0);

        let bar3 = Bar::new().high(9).low(7).close(8).volume(200.0);
        assert_eq!(round(mfi.next(&bar3)), 100.0);

        let bar4 = Bar::new().high(5).low(3).close(4).volume(500.0);
        assert_eq!(round(mfi.next(&bar4)), 65.517);

        let bar5 = Bar::new().high(4).low(2).close(3).volume(5000.0);
        assert_eq!(round(mfi.next(&bar5)), 8.602);

        let bar6 = Bar::new().high(2).low(1).close(1.5).volume(6000.0);
        assert_eq!(round(mfi.next(&bar6)), 0.0);

        let bar7 = Bar::new().high(2).low(2).close(2).volume(7000.0);
        assert_eq!(round(mfi.next(&bar7)), 36.842);

        let bar8 = Bar::new().high(2).low(2).close(2).volume(7000.0);
        assert_eq!(round(mfi.next(&bar8)), 60.87);
    }

    #[test]
    fn test_reset() {
        let mut mfi = MoneyFlowIndex::<3>::new();

        let bar1 = Bar::new().high(3).low(1).close(2).volume(500.0);
        let bar2 = Bar::new().high(2.3).low(2.0).close(2.3).volume(1000.0);

        assert_eq!(round(mfi.next(&bar1)), 50.0);
        assert_eq!(round(mfi.next(&bar2)), 100.0);

        mfi.reset();

        assert_eq!(round(mfi.next(&bar1)), 50.0);
        assert_eq!(round(mfi.next(&bar2)), 100.0);
    }

    #[test]
    fn test_default() {
        MoneyFlowIndex::default();
    }

    #[test]
    fn test_display() {
        let mfi = MoneyFlowIndex::<10>::new();
        assert_eq!(format!("{}", mfi), "MFI(10)");
    }
}
