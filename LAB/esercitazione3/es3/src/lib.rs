pub mod solution;

#[derive(Debug, Clone, PartialEq)]
pub struct CircularBuffer<T> where T : Default {
    pub buffer: Vec<T>,
    pub flags: Vec<bool>,
    pub head: usize,
    pub tail: usize,
    pub size: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CircularBufferError{
    BufferFull,
}

