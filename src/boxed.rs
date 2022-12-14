use alloc::alloc::Global;
use core::{
    alloc::{AllocError, Allocator, Layout},
    mem::MaybeUninit,
    ptr::NonNull,
};

pub struct TryBox<U: ?Sized, A: Allocator = Global> {
    ptr: NonNull<U>,
    allocator: A,
}

impl<T: ?Sized, A: Allocator> TryBox<T, A> {
    /// # Safety
    ///
    /// It is undefined behaviour to use this function to obtain shared mutable references.
    #[inline]
    pub unsafe fn as_nonnull_ptr(slf: &Self) -> NonNull<T> {
        slf.ptr
    }

    #[inline]
    pub unsafe fn leak<'a>(mut slf: Self) -> &'a mut T
    where
        A: 'a,
    {
        slf.ptr.as_mut()
    }
}

impl<T> TryBox<T> {
    #[inline]
    pub fn new(value: T) -> Result<Self, AllocError> {
        Self::new_in(value, Global)
    }

    #[inline]
    pub fn new_uninit() -> Result<TryBox<MaybeUninit<T>>, AllocError> {
        Self::new_uninit_in(Global)
    }
}

impl<T, A: Allocator> TryBox<T, A> {
    #[inline]
    pub fn new_in(value: T, allocator: A) -> Result<Self, AllocError> {
        let layout = Layout::new::<T>();
        allocator
            .allocate(layout)
            .map(|ptr| {
                let ptr = ptr.as_non_null_ptr().cast::<T>();
                unsafe { ptr.as_ptr().write(value) };
                ptr
            })
            .map(|ptr| Self { ptr, allocator })
    }

    #[inline]
    pub fn new_uninit_in(allocator: A) -> Result<TryBox<MaybeUninit<T>, A>, AllocError> {
        let layout = Layout::new::<MaybeUninit<T>>();
        allocator.allocate(layout).map(|ptr| TryBox { ptr: ptr.as_non_null_ptr().cast(), allocator })
    }
}

impl<T, A: Allocator> TryBox<MaybeUninit<T>, A> {
    #[inline]
    pub unsafe fn assume_init(slf: Self) -> TryBox<T, A> {
        TryBox { ptr: slf.ptr.cast(), allocator: slf.allocator }
    }
}

impl<T: Copy> TryBox<[T]> {
    pub fn new_slice(len: usize, value: T) -> Result<Self, AllocError> {
        TryBox::new_slice_in(len, value, Global)
    }
}

impl<T: Copy, A: Allocator> TryBox<[T], A> {
    pub fn new_slice_in(len: usize, value: T, allocator: A) -> Result<Self, AllocError> {
        let mut slice = Self::new_uninit_slice_in(len, allocator)?;
        slice.fill(MaybeUninit::new(value));
        // ### Safety: Slice has been initialized.
        Ok(unsafe { TryBox::<[_], _>::assume_init(slice) })
    }
}

impl<T, A: Allocator> TryBox<[T], A> {
    #[inline]
    pub fn new_uninit_slice_in(len: usize, allocator: A) -> Result<TryBox<[MaybeUninit<T>], A>, AllocError> {
        let layout = Layout::array::<MaybeUninit<T>>(len).map_err(|_| AllocError)?;
        allocator
            .allocate(layout)
            .map(|ptr| TryBox { ptr: NonNull::slice_from_raw_parts(ptr.as_non_null_ptr().cast(), len), allocator })
    }
}

impl<T, A: Allocator> TryBox<[MaybeUninit<T>], A> {
    #[inline]
    pub unsafe fn assume_init(slf: Self) -> TryBox<[T], A> {
        TryBox {
            ptr: NonNull::slice_from_raw_parts(slf.ptr.as_non_null_ptr().cast(), slf.ptr.len()),
            allocator: slf.allocator,
        }
    }
}

impl<U: ?Sized, A: Allocator> core::ops::Deref for TryBox<U, A> {
    type Target = U;

    fn deref(&self) -> &Self::Target {
        // ### Safety: Type constructs initialized values.
        unsafe { self.ptr.as_ref() }
    }
}

impl<U: ?Sized, A: Allocator> core::ops::DerefMut for TryBox<U, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // ### Safety: Type constructs initialized values.
        unsafe { self.ptr.as_mut() }
    }
}
