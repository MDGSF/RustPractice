use std::fmt;
use std::iter::ExactSizeIterator;
use std::iter::FromIterator;
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

pub struct LinkedList<T> {
  head: Option<NonNull<Node<T>>>,
  tail: Option<NonNull<Node<T>>>,
  len: usize,
  marker: PhantomData<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
  pub const fn new() -> Self {
    LinkedList {
      head: None,
      tail: None,
      len: 0,
      marker: PhantomData,
    }
  }

  pub fn push_front(&mut self, elt: T) {
    self.push_front_node(Box::new(Node::new(elt)));
  }

  #[inline]
  fn push_front_node(&mut self, mut node: Box<Node<T>>) {
    unsafe {
      node.next = self.head;
      node.prev = None;
      let node = Some(Box::leak(node).into());

      match self.head {
        None => self.tail = node,
        Some(head) => (*head.as_ptr()).prev = node,
      }

      self.head = node;
      self.len += 1;
    }
  }

  pub fn pop_front(&mut self) -> Option<T> {
    self.pop_front_node().map(Node::into_element)
  }

  #[inline]
  fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
    self.head.map(|node| unsafe {
      let node = Box::from_raw(node.as_ptr());
      self.head = node.next;

      match self.head {
        None => self.tail = None,
        Some(head) => (*head.as_ptr()).prev = None,
      }

      self.len -= 1;
      node
    })
  }

  pub fn push_back(&mut self, elt: T) {
    self.push_back_node(Box::new(Node::new(elt)))
  }

  #[inline]
  fn push_back_node(&mut self, mut node: Box<Node<T>>) {
    unsafe {
      node.next = None;
      node.prev = self.tail;
      let node = Some(Box::leak(node).into());

      match self.tail {
        None => self.head = node,
        Some(tail) => (*tail.as_ptr()).next = node,
      }

      self.tail = node;
      self.len += 1;
    }
  }

  pub fn pop_back(&mut self) -> Option<T> {
    self.pop_back_node().map(Node::into_element)
  }

  #[inline]
  fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
    self.tail.map(|node| unsafe {
      let node = Box::from_raw(node.as_ptr());
      self.tail = node.prev;

      match self.tail {
        None => self.head = None,
        Some(tail) => (*tail.as_ptr()).next = None,
      }

      self.len -= 1;
      node
    })
  }

  pub fn is_empty(&self) -> bool {
    self.head.is_none()
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub fn clear(&mut self) {
    *self = Self::new()
  }

  pub fn front(&self) -> Option<&T> {
    unsafe { self.head.as_ref().map(|node| &node.as_ref().element) }
  }

  pub fn front_mut(&mut self) -> Option<&mut T> {
    unsafe { self.head.as_mut().map(|node| &mut node.as_mut().element) }
  }

  pub fn back(&self) -> Option<&T> {
    unsafe { self.tail.as_ref().map(|node| &node.as_ref().element) }
  }

  pub fn back_mut(&mut self) -> Option<&mut T> {
    unsafe { self.tail.as_mut().map(|node| &mut node.as_mut().element) }
  }

  pub fn contains(&self, x: &T) -> bool
  where
    T: PartialEq<T>,
  {
    self.iter().any(|e| e == x)
  }

  pub fn iter(&self) -> Iter<'_, T> {
    Iter {
      head: self.head,
      tail: self.tail,
      len: self.len,
      marker: PhantomData,
    }
  }

  pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    IterMut {
      head: self.head,
      tail: self.tail,
      len: self.len,
      list: self,
    }
  }
}

impl<T> Drop for LinkedList<T> {
  fn drop(&mut self) {
    struct DropGuard<'a, T>(&'a mut LinkedList<T>);

    impl<'a, T> Drop for DropGuard<'a, T> {
      fn drop(&mut self) {
        while self.0.pop_front_node().is_some() {}
      }
    }

    while let Some(node) = self.pop_front_node() {
      let guard = DropGuard(self);
      drop(node);
      mem::forget(guard);
    }
  }
}

impl<T> Default for LinkedList<T> {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl<T: fmt::Debug> fmt::Debug for LinkedList<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_list().entries(self).finish()
  }
}

impl<T> FromIterator<T> for LinkedList<T> {
  fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
    let mut list = Self::new();

    let list_mut_ref = &mut list;
    iter
      .into_iter()
      .for_each(move |elt| list_mut_ref.push_back(elt));

    list
  }
}

impl<T> IntoIterator for LinkedList<T> {
  type Item = T;
  type IntoIter = IntoIter<T>;

  #[inline]
  fn into_iter(self) -> IntoIter<T> {
    IntoIter { list: self }
  }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
  type Item = &'a T;
  type IntoIter = Iter<'a, T>;

  #[inline]
  fn into_iter(self) -> Iter<'a, T> {
    self.iter()
  }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
  type Item = &'a mut T;
  type IntoIter = IterMut<'a, T>;

  #[inline]
  fn into_iter(self) -> IterMut<'a, T> {
    self.iter_mut()
  }
}

struct Node<T> {
  next: Option<NonNull<Node<T>>>,
  prev: Option<NonNull<Node<T>>>,
  element: T,
}

impl<T> Node<T> {
  fn new(element: T) -> Self {
    Node {
      next: None,
      prev: None,
      element,
    }
  }

  fn into_element(self: Box<Self>) -> T {
    self.element
  }
}

pub struct Iter<'a, T: 'a> {
  head: Option<NonNull<Node<T>>>,
  tail: Option<NonNull<Node<T>>>,
  len: usize,
  marker: PhantomData<&'a Node<T>>,
}

impl<T: fmt::Debug> fmt::Debug for Iter<'_, T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("Iter").field(&self.len).finish()
  }
}

impl<T> Clone for Iter<'_, T> {
  fn clone(&self) -> Self {
    Iter { ..*self }
  }
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  #[inline]
  fn next(&mut self) -> Option<&'a T> {
    if self.len == 0 {
      None
    } else {
      self.head.map(|node| unsafe {
        let node = &*node.as_ptr();
        self.len -= 1;
        self.head = node.next;
        &node.element
      })
    }
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len, Some(self.len))
  }

  #[inline]
  fn last(mut self) -> Option<&'a T> {
    self.next_back()
  }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
  #[inline]
  fn next_back(&mut self) -> Option<&'a T> {
    if self.len == 0 {
      None
    } else {
      self.tail.map(|node| unsafe {
        let node = &*node.as_ptr();
        self.len -= 1;
        self.tail = node.prev;
        &node.element
      })
    }
  }
}

impl<T> ExactSizeIterator for Iter<'_, T> {}

impl<T> FusedIterator for Iter<'_, T> {}

pub struct IterMut<'a, T: 'a> {
  list: &'a mut LinkedList<T>,
  head: Option<NonNull<Node<T>>>,
  tail: Option<NonNull<Node<T>>>,
  len: usize,
}

impl<T: fmt::Debug> fmt::Debug for IterMut<'_, T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("IterMut")
      .field(&self.list)
      .field(&self.len)
      .finish()
  }
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  #[inline]
  fn next(&mut self) -> Option<&'a mut T> {
    if self.len == 0 {
      None
    } else {
      self.head.map(|node| unsafe {
        let node = &mut *node.as_ptr();
        self.len -= 1;
        self.head = node.next;
        &mut node.element
      })
    }
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len, Some(self.len))
  }

  #[inline]
  fn last(mut self) -> Option<&'a mut T> {
    self.next_back()
  }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
  #[inline]
  fn next_back(&mut self) -> Option<&'a mut T> {
    if self.len == 0 {
      None
    } else {
      self.tail.map(|node| unsafe {
        let node = &mut *node.as_ptr();
        self.len -= 1;
        self.tail = node.prev;
        &mut node.element
      })
    }
  }
}

impl<T> ExactSizeIterator for IterMut<'_, T> {}

impl<T> FusedIterator for IterMut<'_, T> {}

pub struct IntoIter<T> {
  list: LinkedList<T>,
}

impl<T: fmt::Debug> fmt::Debug for IntoIter<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("IntoIter").field(&self.list).finish()
  }
}

impl<T> Iterator for IntoIter<T> {
  type Item = T;

  #[inline]
  fn next(&mut self) -> Option<T> {
    self.list.pop_front()
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.list.len, Some(self.list.len))
  }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
  #[inline]
  fn next_back(&mut self) -> Option<T> {
    self.list.pop_back()
  }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

impl<T> FusedIterator for IntoIter<T> {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_list_basic() {
    let mut list1 = LinkedList::new();
    assert_eq!(list1.len(), 0);
    assert_eq!(list1.is_empty(), true);
    assert_eq!(list1.contains(&'a'), false);
    assert_eq!(list1.front(), None);
    assert_eq!(list1.back(), None);

    list1.push_back('a');
    assert_eq!(list1.len(), 1);
    assert_eq!(list1.is_empty(), false);
    assert_eq!(list1.contains(&'a'), true);
    assert_eq!(list1.front(), Some(&'a'));
    assert_eq!(list1.back(), Some(&'a'));

    list1.push_front('z');
    assert_eq!(list1.len(), 2);
    assert_eq!(list1.is_empty(), false);
    assert_eq!(list1.contains(&'a'), true);
    assert_eq!(list1.front(), Some(&'z'));
    assert_eq!(list1.back(), Some(&'a'));

    list1.clear();
    assert_eq!(list1.len(), 0);
    assert_eq!(list1.is_empty(), true);
    assert_eq!(list1.contains(&'a'), false);
    assert_eq!(list1.front(), None);
    assert_eq!(list1.back(), None);
  }

  #[test]
  fn test_list_push_back() {
    let mut list1 = LinkedList::new();
    list1.push_back('a');
    list1.push_back('b');
    list1.push_back('c');
    let mut iter = list1.iter();
    assert_eq!(iter.next(), Some(&'a'));
    assert_eq!(iter.next(), Some(&'b'));
    assert_eq!(iter.next(), Some(&'c'));
    assert_eq!(iter.next(), None);
  }
}
