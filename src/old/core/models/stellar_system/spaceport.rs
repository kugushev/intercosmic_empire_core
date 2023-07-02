use interoptopus::ffi_type;

#[ffi_type]
#[repr(C)]
#[derive(Clone)]
pub struct Spaceport {
    pub orbit_radius: f32,
    pub surface_radius: f32
}
