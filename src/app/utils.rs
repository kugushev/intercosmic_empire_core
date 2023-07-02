use std::mem::MaybeUninit;

pub mod struct_vec;

pub fn zero<T>() -> T {
    unsafe { MaybeUninit::<T>::zeroed().assume_init() }
}

#[derive(Copy, Clone)]
pub struct DeltaTime(f32);

impl DeltaTime {
    pub fn new(seconds: f32) -> Self { Self(seconds) }
    pub fn seconds(&self) -> f32 { self.0 }
}