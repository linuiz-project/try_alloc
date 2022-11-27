use alloc::{alloc::Global, collections::TryReserveError, vec::Vec};
use core::alloc::Allocator;

pub struct TryVec<T, A: Allocator = Global>(Vec<T, A>);

impl<T> Default for TryVec<T> {
    #[inline]
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> TryVec<T> {
    #[inline]
    pub const fn new() -> Self {
        Self(Vec::new())
    }
}

impl<T, A: Allocator> TryVec<T, A> {
    #[inline]
    pub const fn new_in(allocator: A) -> Self {
        Self(Vec::new_in(allocator))
    }

    #[inline]
    pub fn with_capacity_in(capacity: usize, allocator: A) -> Result<Self, TryReserveError> {
        let mut vec = Vec::new_in(allocator);
        vec.try_reserve(capacity)?;
        Ok(Self(vec))
    }

    #[inline]
    pub fn allocator(&self) -> &A {
        self.0.allocator()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.0.as_mut_slice()
    }

    #[inline]
    pub fn iter(&self) -> core::slice::Iter<T> {
        self.as_slice().iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> core::slice::IterMut<T> {
        self.as_mut_slice().iter_mut()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    #[inline]
    pub fn push(&mut self, element: T) -> Result<(), (T, TryReserveError)> {
        if self.len() == self.capacity() && let Err(err) = self.0.try_reserve(1) {
            return Err((element, err));
        }

        self.0.push(element);
        Ok(())
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    #[inline]
    pub fn insert(&mut self, index: usize, element: T) -> Result<(), (T, TryReserveError)> {
        if self.len() == self.capacity() && let Err(err) = self.0.try_reserve(1) {
            return Err((element, err));
        }

        self.0.insert(index, element);
        Ok(())
    }
}

impl<T, A: Allocator> core::ops::Deref for TryVec<T, A> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T, A: Allocator> core::ops::DerefMut for TryVec<T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}
