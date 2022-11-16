use alloc::{alloc::Global, boxed::Box};
use core::{
    alloc::{AllocError, Allocator},
    mem::MaybeUninit,
};

pub struct TryBox<T, A: Allocator = Global>(Box<T, A>);

impl<T> TryBox<T> {
    #[inline]
    pub fn new(value: T) -> Result<Self, AllocError> {
        Box::try_new(value).map(Self)
    }

    #[inline]
    pub fn new_uninit() -> Result<TryBox<MaybeUninit<T>>, AllocError> {
        Box::try_new_uninit().map(|boxed| TryBox(boxed))
    }
}

impl<T, A: Allocator> TryBox<T, A> {
    #[inline]
    pub fn new_in(value: T, allocator: A) -> Result<Self, AllocError> {
        Box::try_new_in(value, allocator).map(Self)
    }

    #[inline]
    pub fn new_uninit_in(allocator: A) -> Result<TryBox<MaybeUninit<T>, A>, AllocError> {
        Box::try_new_uninit_in(allocator).map(|boxed| TryBox(boxed))
    }
}

impl<T, A: Allocator> core::ops::Deref for TryBox<T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T, A: Allocator> core::ops::DerefMut for TryBox<T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}
