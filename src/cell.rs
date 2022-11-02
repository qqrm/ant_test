/// Cell with coords
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FieldCell {
    pub x: i64,
    pub y: i64,
}

impl FieldCell {
    pub fn get_top(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn get_bot(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn get_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn get_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn coords_digits_sum(&self) -> i64 {
        to_digits(self.x).iter().sum::<i64>() + to_digits(self.y).iter().sum::<i64>()
    }
}

/// Convert number to digit vec
fn to_digits(v: i64) -> Vec<i64> {
    let mut v = if v >= 0 { v } else { -v };

    let mut digits = Vec::with_capacity(4);

    while v > 0 {
        let n = v % 10;
        v /= 10;
        digits.push(n);
    }

    digits
}

#[cfg(test)]
mod tests {
    use crate::cell::to_digits;

    #[test]
    fn digit_summ_test_simple() {
        let mut digs = Vec::from([1i64, 0, 0, 0]);
        digs.reverse();
        assert_eq!(to_digits(1000), digs);
    }

    #[test]
    fn digit_summ_test_filled() {
        let mut digs = Vec::from([1i64, 2, 3, 4]);
        digs.reverse();
        assert_eq!(to_digits(1234), digs);
    }

    #[test]
    fn digit_summ_test_semi_filled() {
        let mut digs = Vec::from([1i64, 2]);
        digs.reverse();
        assert_eq!(to_digits(12), digs);
    }
}
