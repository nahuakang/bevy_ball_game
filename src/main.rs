pub mod enemy;
pub mod events;
pub mod player;
pub mod score;
pub mod star;

mod systems;

use bevy::prelude::*;
use enemy::{
    confine_enemy_movement, enemy_movement, spawn_enemies, spawn_enemies_over_time,
    tick_enemy_spawn_timer, update_enemy_direction, EnemySpawnTimer,
};
use events::*;
use player::{
    confine_player_movement, enemy_hit_player, player_hit_star, player_movement, spawn_player,
};
use score::{high_scores_updated, update_high_scores, update_score, HighScores, Score};
use star::{spawn_stars, spawn_stars_over_time, tick_star_spawn_timer, StarSpawnTimer};
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_event::<GameOver>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemies)
        .add_startup_system(spawn_stars)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(enemy_movement)
        .add_system(update_enemy_direction)
        .add_system(confine_enemy_movement)
        .add_system(enemy_hit_player)
        .add_system(player_hit_star)
        .add_system(update_score)
        .add_system(tick_star_spawn_timer)
        .add_system(spawn_stars_over_time)
        .add_system(tick_enemy_spawn_timer)
        .add_system(spawn_enemies_over_time)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .add_system(update_high_scores)
        .add_system(high_scores_updated)
        .run();
}
