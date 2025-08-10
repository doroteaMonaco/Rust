use es3::solution::{CircularBuffer, CircularBufferError};

#[test]
pub fn insert_in_vec(){
    let mut buffer = CircularBuffer::<i32>::new(5);
    buffer.write(1).unwrap();
    buffer.write(2).unwrap();
    buffer.write(3).unwrap();
    buffer.write(4).unwrap();
    buffer.write(5).unwrap();
    assert_eq!(buffer.size(), 5); 
}

#[test]
pub fn read_from_vec(){
    let mut buffer = CircularBuffer::<i32>::new(5);
    buffer.write(1).unwrap();
    buffer.write(2).unwrap();
    buffer.write(3).unwrap();
    buffer.write(4).unwrap();
    buffer.write(5).unwrap();
    assert_eq!(buffer.read(), Some(1));     
}

#[test]
pub fn test_insert_and_check_size() {
    let mut buffer = CircularBuffer::<i32>::new(5);
    buffer.write(1).unwrap();
    assert_eq!(buffer.size(), 1);
}

#[test]
pub fn test_insert_and_read_single_element() {
    let mut buffer = CircularBuffer::<i32>::new(5);
    buffer.write(42).unwrap();
    assert_eq!(buffer.read(), Some(42));
}

#[test]
pub fn test_insert_and_read_multiple_elements() {
    let mut buffer = CircularBuffer::<i32>::new(5);
    for i in 1..=5 {
        buffer.write(i).unwrap();
    }
    for i in 1..=5 {
        assert_eq!(buffer.read(), Some(i));
    }
}

#[test]
pub fn test_head_and_tail_reset() {
    let mut buffer = CircularBuffer::<i32>::new(5);
    for i in 1..=5 {
        buffer.write(i).unwrap();
    }
    for _ in 1..=5 {
        buffer.read();
    }
    assert_eq!(buffer.head, 0);
    assert_eq!(buffer.tail, 0);
}

#[test]
pub fn test_read_from_empty_buffer() {
    let mut buffer = CircularBuffer::<i32>::new(5);
    assert_eq!(buffer.read(), None);
}

#[test]
pub fn test_write_to_full_buffer() {
    let mut buffer = CircularBuffer::<i32>::new(5);
    for i in 1..=5 {
        buffer.write(i).unwrap();
    }
    let result = buffer.write(6);
    assert_eq!(result, Err(CircularBufferError::BufferFull));
}

#[test]
pub fn test_overwrite_on_full_buffer() {
    let mut buffer = CircularBuffer::<i32>::new(5);
    for i in 1..=5 {
        buffer.write(i).unwrap();
    }
    buffer.overwrite(6);
    assert_eq!(buffer.read(), Some(2)); // Element 1 is overwritten
    assert_eq!(buffer.read(), Some(3));
    assert_eq!(buffer.read(), Some(4));
    assert_eq!(buffer.read(), Some(5));
    assert_eq!(buffer.read(), Some(6));
}

#[test]
pub fn test_make_contiguous() {
    let mut buffer = CircularBuffer::<i32>::new(5);
    buffer.write(1).unwrap();
    buffer.write(2).unwrap();
    buffer.read(); 
    buffer.write(3).unwrap();
    buffer.make_contiguous();
    assert_eq!(buffer.head, 0);
    assert_eq!(buffer.tail, 2);
    assert_eq!(buffer.read(), Some(2));
    assert_eq!(buffer.read(), Some(3));
}
