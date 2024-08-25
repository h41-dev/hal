use hal_core::{Trap, TrapDivisionByZero, TrapOverflow};

pub(crate) trait Integer where Self: Sized {
    fn div_checked(self, rhs: Self) -> Result<Self, Trap>;
    fn rem_wrapping(self, rhs: Self) -> Result<Self, Trap>;
}

impl Integer for i32 {
    fn div_checked(self, rhs: Self) -> Result<Self, Trap> {
        self.checked_div(rhs).ok_or_else(|| {
            if self == Self::MIN && rhs == -1 {
                Trap::Overflow(TrapOverflow::Integer)
            } else {
                Trap::DivisionByZero(TrapDivisionByZero::Integer)
            }
        })
    }

    fn rem_wrapping(self, rhs: Self) -> Result<Self, Trap> {
        if rhs == 0 {
            Err(Trap::DivisionByZero(TrapDivisionByZero::Integer))
        } else {
            Ok(self.wrapping_rem(rhs))
        }
    }
}

impl Integer for u32 {
    fn div_checked(self, rhs: Self) -> Result<Self, Trap> {
        self.checked_div(rhs).ok_or_else(|| {
            Trap::DivisionByZero(TrapDivisionByZero::Integer)
        })
    }

    fn rem_wrapping(self, rhs: Self) -> Result<Self, Trap> {
        if rhs == 0 {
            Err(Trap::DivisionByZero(TrapDivisionByZero::Integer))
        } else {
            Ok(self % rhs)
        }
    }
}

impl Integer for i64 {
    fn div_checked(self, rhs: Self) -> Result<Self, Trap> {
        self.checked_div(rhs).ok_or_else(|| {
            if self == Self::MIN && rhs == -1 {
                Trap::Overflow(TrapOverflow::Integer)
            } else {
                Trap::DivisionByZero(TrapDivisionByZero::Integer)
            }
        })
    }

    fn rem_wrapping(self, rhs: Self) -> Result<Self, Trap> {
        if rhs == 0 {
            Err(Trap::DivisionByZero(TrapDivisionByZero::Integer))
        } else {
            Ok(self.wrapping_rem(rhs))
        }
    }
}

impl Integer for u64 {
    fn div_checked(self, rhs: Self) -> Result<Self, Trap> {
        self.checked_div(rhs).ok_or_else(|| {
            Trap::DivisionByZero(TrapDivisionByZero::Integer)
        })
    }

    fn rem_wrapping(self, rhs: Self) -> Result<Self, Trap> {
        if rhs == 0 {
            Err(Trap::DivisionByZero(TrapDivisionByZero::Integer))
        } else {
            Ok(self % rhs)
        }
    }
}