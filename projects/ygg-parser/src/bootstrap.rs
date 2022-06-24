#[repr(usize)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum YggdrasilType {
    Program = 0,
    Statement = 1,
    Expression = 2,
}

impl From<usize> for YggdrasilType {
    fn from(value: usize) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl Into<usize> for YggdrasilType {
    fn into(self) -> usize {
        self as usize
    }
}
