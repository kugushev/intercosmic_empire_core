use interoptopus::ffi_type;

pub static CONSTANTS: Constants = Constants {
    gap_between_waypoints: 0.05
};

#[ffi_type]
#[repr(C)]
#[derive(Clone)]
pub struct Constants {
    pub gap_between_waypoints: f32
}

