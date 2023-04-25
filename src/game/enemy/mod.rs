use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; // Sprite size
pub const NUMBER_OF_ENEMIES: usize = 4;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            // Enter State Systems
            // spawn_enemies only runs once when we enter AppState::Game
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            // The following systems only run in AppState::Game and SimulationState::Running
            .add_systems(
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // Exit State Systems
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}
