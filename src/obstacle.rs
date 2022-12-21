use crate::ICEVector3;
use interoptopus::ffi_type;

#[ffi_type]
#[repr(C)]
pub struct Obstacle {
    pub position: ICEVector3,
    pub radius: f32,
}