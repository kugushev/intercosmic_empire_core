use std::mem::MaybeUninit;

pub mod struct_vec;
pub mod guard;
pub mod interop_logger;
pub mod random;
pub mod delta_time;
pub mod translation;
pub mod repr_test;
pub mod quat_extra;

pub fn zero<T>() -> T {
    unsafe { MaybeUninit::<T>::zeroed().assume_init() }
}