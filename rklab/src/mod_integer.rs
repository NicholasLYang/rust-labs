use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::cmp::PartialEq;
const PRIME: i64 = 961748941;

#[derive(Copy, Clone)]
pub struct ModInteger (i64);

impl PartialEq for ModInteger {
    fn eq(self: &ModInteger, other: &ModInteger) -> bool {
        return self.0 == other.0;
    }
}

impl Add for ModInteger {
    type Output = ModInteger;

    fn add(self, other: ModInteger) -> ModInteger {
        ModInteger((self.0 + other.0) % PRIME)
    }
}

impl Sub for ModInteger {
    type Output = ModInteger;
    fn sub(self, other: ModInteger) -> ModInteger {
        ModInteger(
            if self.0 > other.0 { self.0 - other.0 } else { self.0 + PRIME - other.0 }
        )
    }
}

impl Mul for ModInteger {
    type Output = ModInteger;
    fn mul(self, other: ModInteger) -> ModInteger {
        ModInteger(
            (self.0 * other.0) % PRIME
        )
    }
}

