[![Build Status](https://travis-ci.com/immortalinfidel/aroon-rs.svg?branch=master)](https://travis-ci.com/immortalinfidel/aroon-rs)

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->


- [AROON](#aroon)
    - [Calculation](#calculation)
- [AROONOSC](#aroonosc)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->


<a name="aroonmd"></a>

# AROON 
```
use ta_common::traits::Indicator;
use aroon_rs::AROON;
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
```
### Calculation
aroon_up=100*(n-h)/n;  
aroon_down=100*(n-l)/n;  
where:  
n=period  
h= highest days ago  
l=lowest days ago


<a name="aroonoscmd"></a>

# AROONOSC
```

use ta_common::traits::Indicator;
use aroon_rs::AROONOSC;

let mut aroonosc = AROONOSC::new(5);
assert_eq!(aroonosc.next([82.15, 81.29, ]), None);
assert_eq!(aroonosc.next([81.89, 80.64, ]), None);
assert_eq!(aroonosc.next([83.03, 81.31, ]), None);
assert_eq!(aroonosc.next([83.30, 82.65, ]), None);
assert_eq!(aroonosc.next([83.85, 83.07, ]), None);
assert_eq!(aroonosc.next([83.90, 83.11, ]), Some(80.00));
assert_eq!(aroonosc.next([83.33, 82.49, ]), Some(60.00));
assert_eq!(aroonosc.next([84.30, 82.30, ]), Some(0.00));
assert_eq!(aroonosc.next([84.84, 84.15, ]), Some(20.00));
assert_eq!(aroonosc.next([85.00, 84.11, ]), Some(40.00));
assert_eq!(aroonosc.next([85.90, 84.03, ]), Some(60.00));
assert_eq!(aroonosc.next([86.58, 85.39, ]), Some(80.00));
assert_eq!(aroonosc.next([86.98, 85.76, ]), Some(40.00));
assert_eq!(aroonosc.next([88.00, 87.17, ]), Some(60.00));
assert_eq!(aroonosc.next([87.87, 87.01, ]), Some(60.00));
```