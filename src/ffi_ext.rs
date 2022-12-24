use interoptopus::lang::c::{CompositeType, CType, Field, PrimitiveType};

pub fn vec3_read_ptr() -> CType {
    CType::ReadPointer(Box::new(vec3()))
}

pub fn vec3_read_write_ptr() -> CType {
    CType::ReadWritePointer(Box::new(vec3()))
}

fn vec3() -> CType {
    let fields = vec![
        Field::new("x".to_string(), CType::Primitive(PrimitiveType::F32)),
        Field::new("y".to_string(), CType::Primitive(PrimitiveType::F32)),
        Field::new("z".to_string(), CType::Primitive(PrimitiveType::F32)),
    ];

    let composite = CompositeType::new("Vec3".to_string(), fields);
    CType::Composite(composite)
}