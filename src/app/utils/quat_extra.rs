use glam::{Quat, Vec3};
use interoptopus::{ffi_function, ffi_surrogates};
use crate::ffi::utils::FFIQuat;
use crate::ffi::surrogates::vec3;

#[ffi_function]
#[ffi_surrogates(forward = "vec3", up = "vec3")]
#[no_mangle]
pub extern "C" fn ice_test_quat_look_rotation(forward: Vec3, up: Vec3) -> FFIQuat {
    let quat = forward.look_rotation(up);
    quat.into()
}

pub const VEC3_UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
pub const VEC3_DOWN: Vec3 = Vec3::new(0.0, -1.0, 0.0);
pub const VEC3_FORWARD: Vec3 = Vec3::new(0.0, 0.0, 1.0);

pub trait QuatEx {
    fn look_rotation(&self, up: Vec3) -> Quat;
}

impl QuatEx for Vec3 {
    // https://discussions.unity.com/t/what-is-the-source-code-of-quaternion-lookrotation/72474
    fn look_rotation(&self, up: Vec3) -> Quat {
        let forward = self.normalize();

        let vector = forward;
        let vector2 = up.cross(vector).normalize();
        let vector3 = vector.cross(vector2);
        let m00 = vector2.x;
        let m01 = vector2.y;
        let m02 = vector2.z;
        let m10 = vector3.x;
        let m11 = vector3.y;
        let m12 = vector3.z;
        let m20 = vector.x;
        let m21 = vector.y;
        let m22 = vector.z;


        let num8 = (m00 + m11) + m22;
        let mut quaternion = Quat::default();
        if num8 > 0.0
        {
            let mut num = (num8 + 1.0).sqrt();
            quaternion.w = num * 0.5;
            num = 0.5 / num;
            quaternion.x = (m12 - m21) * num;
            quaternion.y = (m20 - m02) * num;
            quaternion.z = (m01 - m10) * num;
            return quaternion;
        }
        if (m00 >= m11) && (m00 >= m22)
        {
            let num7 = (((1.0 + m00) - m11) - m22).sqrt();
            let num4 = 0.5 / num7;
            quaternion.x = 0.5 * num7;
            quaternion.y = (m01 + m10) * num4;
            quaternion.z = (m02 + m20) * num4;
            quaternion.w = (m12 - m21) * num4;
            return quaternion;
        }
        if m11 > m22
        {
            let num6 = (((1.0 + m11) - m00) - m22).sqrt();
            let num3 = 0.5 / num6;
            quaternion.x = (m10 + m01) * num3;
            quaternion.y = 0.5 * num6;
            quaternion.z = (m21 + m12) * num3;
            quaternion.w = (m20 - m02) * num3;
            return quaternion;
        }

        let num5 = (((1.0 + m22) - m00) - m11).sqrt();
        let num2 = 0.5 / num5;
        quaternion.x = (m20 + m02) * num2;
        quaternion.y = (m21 + m12) * num2;
        quaternion.z = 0.5 * num5;
        quaternion.w = (m01 - m10) * num2;

        quaternion
    }
}


