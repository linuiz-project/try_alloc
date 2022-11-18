#![cfg_attr(not(test), no_std)]
#![feature(
    allocator_api,                  // #32838 <https://github.com/rust-lang/rust/issues/32838>
    let_chains,                     // #53667 <https://github.com/rust-lang/rust/issues/53667>
    slice_ptr_get,                  // #74265 <https://github.com/rust-lang/rust/issues/74265
    nonnull_slice_from_raw_parts,   // #74265 <https://github.com/rust-lang/rust/issues/74265>
)]

extern crate alloc;

pub mod vec;
pub mod boxed;
