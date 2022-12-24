pub trait Index: Copy {
    fn index(self) -> usize;
}

impl Index for u32 {
    fn index(self) -> usize {
        self as _
    }
}

impl Index for u64 {
    fn index(self) -> usize {
        self as _
    }
}

impl Index for usize {
    fn index(self) -> usize {
        self as _
    }
}
