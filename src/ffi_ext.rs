use interoptopus::lang::c::{CompositeType, CType, Documentation, Meta};

pub fn vec3_read_ptr() -> CType {
    CType::ReadPointer(Box::new(vec3()))
}

pub fn vec3_read_write_ptr() -> CType {
    CType::ReadWritePointer(Box::new(vec3()))
}

pub fn vec3() -> CType {
    let composite = CompositeType::with_meta("Vector3".to_string(), vec![],
                                             Meta::with_namespace_documentation(
                                                 "UnityEngine".to_string(),
                                                 Documentation::new()));
    CType::Composite(composite)
}