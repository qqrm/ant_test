/// Represents a cell in a field with x and y coordinates.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FieldCell {
    pub x: i64,
    pub y: i64,
}

impl FieldCell {
    /// Gets the cell that is directly above this one.
    pub fn get_top(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    /// Gets the cell that is directly below this one.
    pub fn get_bot(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    /// Gets the cell that is directly to the left of this one.
    pub fn get_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    /// Gets the cell that is directly to the right of this one.
    pub fn get_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    /// Computes the sum of all digits of the x and y coordinates.
    pub fn coords_digits_sum(&self) -> i64 {
        self.x.abs().sum() + self.y.abs().sum()
    }
}

/// Extension trait for calculating the digit sum of an i64 number.
trait DigitSum {
    fn sum(&self) -> i64;
}

impl DigitSum for i64 {
    fn sum(&self) -> i64 {
        let mut n = self.abs();
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digit_summ_test_simple() {
        assert_eq!(1000.sum(), 1);
    }

    #[test]
    fn digit_summ_test_filled() {
        assert_eq!(1234.sum(), 10);
    }

    #[test]
    fn digit_summ_test_semi_filled() {
        assert_eq!(12.sum(), 3);
    }
}
