use intercosmic_empire::ffi::ice_get_battle_stellar_system_view_model;
use intercosmic_empire::ffi_models::FFIOutcome;
use intercosmic_empire::game::game_context::GameContext;

#[test]
fn ice_get_battle_stellar_system_view_model_no_battle() {
    let mut ctx = GameContext::new();
    let result = ice_get_battle_stellar_system_view_model(&mut ctx);
    assert_eq!(result.outcome, FFIOutcome::Unable)
}