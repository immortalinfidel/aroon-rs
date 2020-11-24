use ta_common::fixed_queue::FixedQueue;
use ta_common::traits::Indicator;
#[doc(include = "../docs/aroon.md")]
pub struct AROON {
    high_history: FixedQueue<f64>,
    low_history: FixedQueue<f64>,
    period: u32,
    index: u32,
}

impl AROON {
    pub fn new(period: u32) -> AROON {
        Self {
            high_history: FixedQueue::new(period),
            low_history: FixedQueue::new(period),
            period,
            index: 0,
        }
    }

    fn get_min_ago(&self) -> i32 {
        let mut min_index = 0;
        let mut min = self.low_history.at(min_index).unwrap();

        for i in 1..self.low_history.size() as i32 {
            let curr = self.low_history.at(i).unwrap();
            if curr <= min {
                min = curr;
                min_index = i;
            }
        }

        return self.period as i32 - (min_index) - 1;
    }
    fn get_max_ago(&self) -> i32 {
        let mut max_index = 0;
        let mut max = self.high_history.at(max_index).unwrap();

        for i in 1..self.high_history.size() as i32 {
            let curr = self.high_history.at(i).unwrap();
            if curr >= max {
                max = curr;
                max_index = i;
            }
        }

        return self.period as i32 - (max_index) - 1;
    }
}


impl Indicator<[f64; 2], Option<[f64; 2]>> for AROON {
    fn next(&mut self, input: [f64; 2]) -> Option<[f64; 2]> {
        let high = input[0];
        let low = input[1];
        self.low_history.add(low);
        self.high_history.add(high);
        self.index = self.index + 1;

        return if self.index > self.period {
            let period = self.period as f64;
            let lago = self.get_min_ago() as f64;
            let down = 100_f64 * (period - lago) / period;
            let hago = self.get_max_ago() as f64;
            let up = 100_f64 * (period - hago) / period;
            Some([down, up])
        } else {
            None
        };
    }

    fn reset(&mut self) {
        self.high_history.clear();
        self.low_history.clear();
        self.index = 0;
    }
}


#[cfg(test)]
mod tests {
    use ta_common::traits::Indicator;
    use crate::aroon::AROON;

    #[test]
    fn aaron_works() {
        let mut aroon = AROON::new(5);
        assert_eq!(aroon.next([82.15, 81.29]), None);
        assert_eq!(aroon.next([81.89, 80.64]), None);
        assert_eq!(aroon.next([83.03, 81.31]), None);
        assert_eq!(aroon.next([83.30, 82.65]), None);
        assert_eq!(aroon.next([83.85, 83.07]), None);
        assert_eq!(aroon.next([83.90, 83.11]), Some([20.00, 100.00]));
        assert_eq!(aroon.next([83.33, 82.49]), Some([20.00, 80.00]));
        assert_eq!(aroon.next([84.30, 82.30]), Some([100.00, 100.00]));
        assert_eq!(aroon.next([84.84, 84.15]), Some([80.00, 100.00]));
        assert_eq!(aroon.next([85.00, 84.11]), Some([60.00, 100.00]));
        assert_eq!(aroon.next([85.90, 84.03]), Some([40.00, 100.00]));
        assert_eq!(aroon.next([86.58, 85.39]), Some([20.00, 100.00]));
        assert_eq!(aroon.next([86.98, 85.76]), Some([60.00, 100.00]));
        assert_eq!(aroon.next([88.00, 87.17]), Some([40.00, 100.00]));
        assert_eq!(aroon.next([87.87, 87.01]), Some([20.00, 80.00]));
    }
}
