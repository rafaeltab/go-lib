pub trait FreeLeftShift {
    fn free_shl(self, rhs: u32) -> Self;
}

impl FreeLeftShift for u64 {
    fn free_shl(self, rhs: u32) -> Self {
        self.checked_shl(rhs).unwrap_or(0)
    }
}

pub trait FreeRightShift {
    fn free_shr(self, rhs: u32) -> Self;
}

impl FreeRightShift for u64 {
    fn free_shr(self, rhs: u32) -> Self {
        self.checked_shr(rhs).unwrap_or(0)
    }
}
