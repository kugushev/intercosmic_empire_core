use bevy_ecs::prelude::{Mut, Resource, World};

pub fn resources_and_non_send_mut<T1: Resource, T2: Resource, T3: 'static>(world: &mut World) -> (Mut<'_, T1>, Mut<'_, T2>, Mut<'_, T3>) {
    let cell = world.as_unsafe_world_cell();
    unsafe {
        (cell.get_resource_mut::<T1>().expect("First resource not found"),
         cell.get_resource_mut::<T2>().expect("Second resource not found"),
         cell.get_non_send_resource_mut::<T3>().expect("Third resource not found"))
    }
}
