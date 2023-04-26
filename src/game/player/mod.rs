use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

use crate::AppState;

use super::SimulationState;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // Sprite size

#[derive(SystemSet, Clone, Debug, Eq, Hash, PartialEq)]
pub enum PlayerSystemSet {
    Movement,
    Confinement,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement))
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            // .add_system(
            //     player_movement
            //         .in_set(PlayerSystemSet::Movement)
            //         .run_if(in_state(AppState::Game))
            //         .run_if(in_state(SimulationState::Running)),
            // )
            // .add_system(
            //     confine_player_movement
            //         .in_set(PlayerSystemSet::Confinement)
            //         .run_if(in_state(AppState::Game))
            //         .run_if(in_state(SimulationState::Running)),
            // )
            .add_systems(
                (
                    player_movement.in_set(PlayerSystemSet::Movement),
                    confine_player_movement.in_set(PlayerSystemSet::Confinement),
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_systems(
                (enemy_hit_player, player_hit_star)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
