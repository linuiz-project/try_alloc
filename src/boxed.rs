use alloc::{alloc::Global, boxed::Box};
use core::{
    alloc::{AllocError, Allocator},
    mem::MaybeUninit,
    ptr::NonNull,
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

    /// # Safety
    ///
    /// It is undefined behaviour to use this function to obtain shared mutable references.
    #[inline]
    pub unsafe fn as_nonnull_ptr(slf: Self) -> NonNull<T> {
        NonNull::from(&**slf)
    }
}

impl<T, A: Allocator> core::ops::Deref for TryBox<T, A> {
    type Target = Box<T, A>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, A: Allocator> core::ops::DerefMut for TryBox<T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
