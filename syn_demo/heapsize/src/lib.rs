use std::mem;

pub use heapsize_derive::*;

pub trait HeapSize {
    fn heap_size_of_children(&self) -> usize;
}

impl HeapSize for u8 {
    fn heap_size_of_children(&self) -> usize {
        0
    }
}

impl HeapSize for String {
    fn heap_size_of_children(&self) -> usize {
        self.capacity()
    }
}

impl<T> HeapSize for Box<T>
where
    T: ?Sized + HeapSize,
{
    fn heap_size_of_children(&self) -> usize {
        mem::size_of_val(&**self) + (**self).heap_size_of_children()
    }
}

impl<T> HeapSize for [T]
where
    T: HeapSize,
{
    fn heap_size_of_children(&self) -> usize {
        self.iter().map(HeapSize::heap_size_of_children).sum()
    }
}

impl<'a, T> HeapSize for &'a T
where
    T: ?Sized,
{
    fn heap_size_of_children(&self) -> usize {
        0
    }
}
