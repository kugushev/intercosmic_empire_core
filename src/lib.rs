#[no_mangle]
pub extern "C" fn hello_from_rust(a: i32) -> i32 {
    a + a + 42
}
fn main() {}
