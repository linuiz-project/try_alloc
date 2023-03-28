#![cfg_attr(not(test), no_std)]
#![feature(
    allocator_api,                  // #32838 <https://github.com/rust-lang/rust/issues/32838>
    let_chains,                     // #53667 <https://github.com/rust-lang/rust/issues/53667>
    slice_ptr_get,                  // #74265 <https://github.com/rust-lang/rust/issues/74265>
    error_in_core,                  // #103765 <https://github.com/rust-lang/rust/issues/103765>
    try_trait_v2,                   // #84277 <https://github.com/rust-lang/rust/issues/84277>
    control_flow_enum,              // #75744 <https://github.com/rust-lang/rust/issues/75744>
    never_type,                     // #35121 <https://github.com/rust-lang/rust/issues/35121>
    associated_type_defaults,       // #29661 <https://github.com/rust-lang/rust/issues/29661>
)]

extern crate alloc;

pub mod boxed;
pub mod vec;

use core::{
    convert::Infallible,
    error::Error,
    ops::{ControlFlow, Try},
};

pub trait PanicStrategy<E: Error> {
    type ReturnType: Try;

    fn handle<T: Try<Residual = Result<Infallible, E>>>(t: T) -> Self::ReturnType;
    fn ok() -> Self::ReturnType;
}

pub struct Panic;
impl<E: Error> PanicStrategy<E> for Panic {
    type ReturnType = ControlFlow<!>;

    #[inline(always)]
    fn handle<T: Try<Residual = Result<Infallible, E>>>(t: T) -> Self::ReturnType {
        match t.branch() {
            ControlFlow::Continue(_) => ControlFlow::Continue(()),
            ControlFlow::Break(b) => panic!("{:?}", unsafe { b.unwrap_err_unchecked() }),
        }
    }

    #[inline]
    fn ok() -> Self::ReturnType {
        ControlFlow::Continue(())
    }
}

pub struct Recover;
impl<E: Error> PanicStrategy<E> for Recover {
    type ReturnType = ControlFlow<E>;

    #[inline]
    fn handle<T: Try<Residual = Result<Infallible, E>>>(t: T) -> Self::ReturnType {
        match t.branch() {
            ControlFlow::Continue(_) => ControlFlow::Continue(()),
            ControlFlow::Break(b) => ControlFlow::Break(unsafe { b.unwrap_err_unchecked() }),
        }
    }

    #[inline]
    fn ok() -> Self::ReturnType {
        ControlFlow::Continue(())
    }
}
