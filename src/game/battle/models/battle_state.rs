use bevy_ecs::prelude::Resource;

use crate::game::battle::components::stellar::warp_gate::WarpGate;

#[derive(Default, Resource)]
pub struct BattleState {
    pub warp_gates: Vec<WarpGate>,
}