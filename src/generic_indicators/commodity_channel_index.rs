use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::generic_indicators::{MeanAbsoluteDeviation, SimpleMovingAverage};
use crate::{Close, High, Low, Next, Period, Reset};

/// Commodity Channel Index (CCI)
///
/// The commodity channel index is an oscillator originally introduced by Donald Lambert in 1980.
///
/// Since its introduction, the indicator has grown in popularity and is now a very common tool for
/// traders in identifying cyclical trends not only in commodities but also equities and currencies.
/// The CCI can be adjusted to the timeframe of the market traded on by changing the averaging period.
///
/// # Formula
///
/// CCI(_period_) = (TP - SMA(_period_) of TP) / (MAD(_period_) * 0.015)
///
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0). Default is 20.
///
/// # Links
///
/// * [Commodity Channel Index, Wikipedia](https://en.wikipedia.org/wiki/Commodity_channel_index)
/// * [Commodity Channel Index, StockCharts](https://school.stockcharts.com/doku.php?id=technical_indicators:commodity_channel_index_cci)
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct CommodityChannelIndex<const N: usize = 20> {
    sma: SimpleMovingAverage<N>,
    mad: MeanAbsoluteDeviation<N>,
}

impl<const N: usize> CommodityChannelIndex<N> {
    pub fn new() -> Self {
        Self {
            sma: SimpleMovingAverage::new(),
            mad: MeanAbsoluteDeviation::new(),
        }
    }
}

impl<const N: usize> Period for CommodityChannelIndex<N> {
    fn period(&self) -> usize {
        self.sma.period()
    }
}

impl<T: Close + High + Low, const N: usize> Next<&T> for CommodityChannelIndex<N> {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        let tp = (input.close() + input.high() + input.low()) / 3.0;
        let sma = self.sma.next(tp);
        let mad = self.mad.next(input);

        if mad == 0.0 {
            return 0.0;
        }

        (tp - sma) / (mad * 0.015)
    }
}

impl<const N: usize> Reset for CommodityChannelIndex<N> {
    fn reset(&mut self) {
        self.sma.reset();
        self.mad.reset();
    }
}

impl Default for CommodityChannelIndex<20> {
    fn default() -> Self {
        CommodityChannelIndex::new()
    }
}

impl<const N: usize> fmt::Display for CommodityChannelIndex<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CCI({})", N)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    #[test]
    fn test_next_bar() {
        let mut cci = CommodityChannelIndex::<5>::new();

        let bar1 = Bar::new().high(2).low(1).close(1.5);
        assert_eq!(round(cci.next(&bar1)), 0.0);

        let bar2 = Bar::new().high(5).low(3).close(4);
        assert_eq!(round(cci.next(&bar2)), 66.667);

        let bar3 = Bar::new().high(9).low(7).close(8);
        assert_eq!(round(cci.next(&bar3)), 100.0);

        let bar4 = Bar::new().high(5).low(3).close(4);
        assert_eq!(round(cci.next(&bar4)), -13.793);

        let bar5 = Bar::new().high(5).low(3).close(4);
        assert_eq!(round(cci.next(&bar5)), -13.514);

        let bar6 = Bar::new().high(2).low(1).close(1.5);
        assert_eq!(round(cci.next(&bar6)), -126.126);
    }

    #[test]
    fn test_reset() {
        let mut cci = CommodityChannelIndex::<5>::new();

        let bar1 = Bar::new().high(2).low(1).close(1.5);
        let bar2 = Bar::new().high(5).low(3).close(4);

        assert_eq!(round(cci.next(&bar1)), 0.0);
        assert_eq!(round(cci.next(&bar2)), 66.667);

        cci.reset();

        assert_eq!(round(cci.next(&bar1)), 0.0);
        assert_eq!(round(cci.next(&bar2)), 66.667);
    }

    #[test]
    fn test_default() {
        CommodityChannelIndex::default();
    }

    #[test]
    fn test_display() {
        let indicator = CommodityChannelIndex::<10>::new();
        assert_eq!(format!("{}", indicator), "CCI(10)");
    }
}
