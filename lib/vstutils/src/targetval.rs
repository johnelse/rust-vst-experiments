extern crate num_traits;

use self::num_traits::Num;

/// TargetVal

pub enum Rate<T> {
    Absolute(T),
    Relative(T),
}

pub struct TargetVal<T: Copy+Num+PartialOrd> {
    inc_rate: Rate<T>,
    dec_rate: Rate<T>,
    target:   T,
    value:    T,
}

impl<T: Copy+Num+PartialOrd> TargetVal<T> {
    pub fn new(inc_rate: Rate<T>, dec_rate: Rate<T>, value: T) -> TargetVal<T> {
        TargetVal {
            inc_rate: inc_rate,
            dec_rate: dec_rate,
            target:   value,
            value:    value,
        }
    }

    pub fn set_inc_rate(&mut self, inc_rate: Rate<T>) {
        self.inc_rate = inc_rate;
    }

    pub fn set_dec_rate(&mut self, dec_rate: Rate<T>) {
        self.dec_rate = dec_rate;
    }

    pub fn get_target(&self) -> &T {
        &self.target
    }

    pub fn set_target(&mut self, target: T) {
        self.target = target;
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn advance(&mut self) {
        if self.target > self.value {
            match self.inc_rate {
                Rate::Absolute(rate) =>
                    self.value = self.value + rate,
                Rate::Relative(rate) =>
                    self.value = self.value + (self.target - self.value) * rate,
            }
        }
        else if self.target < self.value {
            match self.dec_rate {
                Rate::Absolute(rate) =>
                    self.value = self.value - rate,
                Rate::Relative(rate) =>
                    self.value = self.value - (self.value - self.target) * rate,
            }
        }
    }
}
