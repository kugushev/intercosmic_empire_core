use crate::game::battle::ecs_context::EcsContext;
use crate::game::battle::models::battle_parameters::BattleParameters;
use crate::game::battle::models::battle_state::BattleState;
use interoptopus::ffi_type;
use crate::game::battle::factories::spaceship_factory::create_spaceship;
use crate::game::battle::models::route::Route;
use crate::game::battle::models::warp_gate::WarpGate;
use crate::game::battle::utils::ecs_ext::resources_and_non_send_mut;
use crate::game::battle::views::BattleViewsState;
use crate::game::core::models::faction::Faction;
use crate::game::core::models::spaceships::spaceship_mark::SpaceshipMark;
use crate::game::core::models::spaceships::spaceship_parameters::SpaceshipParameters;
use crate::game::core::models::stellar_system::StellarSystem;
use crate::game::core::models::stellar_system::production::Productive;
use crate::game::utils::interop_logger::LoggerRef;
use crate::log;

#[ffi_type(opaque)]
pub struct BattleContext {
    pub(crate) ecs: EcsContext,
}

pub const WARP_GATE_ID_SHIFT: i32 = 1000;

impl BattleContext {
    pub(crate) fn new(battle_parameters: BattleParameters, stellar_system: StellarSystem,
                      logger: LoggerRef) -> BattleContext {
        let mut ecs = EcsContext::new(logger);

        ecs.world.insert_resource(battle_parameters);
        ecs.world.insert_resource(stellar_system);
        ecs.world.insert_resource(BattleState::default());
        ecs.world.insert_resource(BattleViewsState::default());

        BattleContext { ecs }
    }

    pub(crate) fn get_battle_state(&self) -> &BattleState {
        self.ecs.world.resource::<BattleState>()
    }
    pub(crate) fn get_battle_view_state(&self) -> &BattleViewsState {
        self.ecs.world.resource::<BattleViewsState>()
    }
    pub(crate) fn get_stellar_system(&self) -> &StellarSystem {
        self.ecs.world.resource::<StellarSystem>()
    }
    pub(crate) fn open_warp_gate(&mut self, warp_gate: WarpGate) -> i32 {
        // todo: assert distance between planet and other warp gates

        let mut battle_state = self.ecs.world.resource_mut::<BattleState>();
        let len: i32 = battle_state.warp_gates.len().try_into().expect("Unconvertable len to i32");
        battle_state.warp_gates.push(warp_gate);
        len + WARP_GATE_ID_SHIFT // use magic shift to prevent id overlapping with planets
    }

    pub fn spawn_spaceship(&mut self, faction: &Faction, spawner_id: i32, route: Route,
                           mark: SpaceshipMark) {
        let parameters = SpaceshipParameters::get_parameters(&mark);

        let ecs_world = &mut self.ecs.world;
        let (mut stellar_system, mut battle_state, logger) =
            resources_and_non_send_mut::<StellarSystem, BattleState, LoggerRef>(ecs_world);
        let found = stellar_system.planets.iter_mut().find(|p| { p.info.id.0 == spawner_id }); // todo: refactor spawner search, cause  0(n) is 'fine', but not good
        let success = match found {
            Some(p) => {
                let produced = p.try_produce(parameters.cost);
                log!(logger, format!("Produced={produced} from planet {} {}/{}", p.info.id.0, p.current_product, parameters.cost));
                produced
            }
            None => {
                let warp_gates = &mut battle_state.warp_gates;
                let id = usize::try_from(spawner_id - WARP_GATE_ID_SHIFT).expect("Unconvertable spawner_id to usize");
                assert!(id < warp_gates.len());
                let warp_gate = &mut warp_gates[id];
                let produced = warp_gate.try_produce(parameters.cost);
                log!(logger, format!("Produced={produced} from warp gate {id} {}/{}", warp_gate.current_product, parameters.cost));
                produced
            }
        };

        if success {
            log!(logger, format!("Create spaceship mark {} {}", mark.clone() as i32, faction.clone() as i32));
            create_spaceship(ecs_world, route, faction.clone(), mark);
        }
    }
}