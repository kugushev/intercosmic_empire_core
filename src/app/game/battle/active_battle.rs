use crate::app::game::battle::entities::stellar_system::StellarSystem;
use crate::app::game::core::faction::Faction;
use crate::app::game::core::stellar_system::StellarSystemInfo;
use crate::app::utils::DeltaTime;

pub struct ActiveBattle {
    stellar_system: StellarSystem,
}

impl ActiveBattle {
    pub fn new(stellar_system_info: StellarSystemInfo, stellar_system_faction: Faction) -> Self {
        Self {
            stellar_system: StellarSystem::new(stellar_system_info, stellar_system_faction)
        }
    }

    pub fn update(&mut self, delta: DeltaTime) {
        self.stellar_system.update(delta);
    }
}

#[cfg(test)]
mod tests {
    use crate::app::game::battle::active_battle::ActiveBattle;
    use crate::app::game::core::faction::Faction;
    use crate::app::game::core::stellar_system::StellarSystemInfo;
    use crate::app::utils::DeltaTime;

    #[test]
    fn production_on_update() {
        let mut battle = ActiveBattle::new(StellarSystemInfo::default(), Faction::Red);

        assert_current_product(&mut battle, (0.0, 0.0, 0.0));

        battle.update(DeltaTime::new(1.0));
        assert_current_product(&mut battle, (0.3, 1.5, 5.0));

        battle.update(DeltaTime::new(0.5));
        assert_current_product(&mut battle, (0.3 + 0.15, 1.5 + 0.75, 5.0 + 2.5));

        battle.update(DeltaTime::new(2.0));
        assert_current_product(&mut battle, (0.3 + 0.15 + 0.6, 1.5 + 0.75 + 3.0, 5.0 + 2.5 + 10.0));

        fn assert_current_product(battle: &mut ActiveBattle, expected: (f32, f32, f32)) {
            let mut iter = battle.stellar_system.planets.iter_mut();
            let mercury = iter.next().unwrap().current_product;
            let earth = iter.next().unwrap().current_product;
            let jupiter = iter.next().unwrap().current_product;

            assert_eq!(mercury, expected.0);
            assert_eq!(earth, expected.1);
            assert_eq!(jupiter, expected.2);
        }
    }


    // todo: max production case

    // todo: abandoned planet case
}