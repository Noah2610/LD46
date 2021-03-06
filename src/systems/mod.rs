pub mod prelude {
    pub use super::control_player::ControlPlayerSystem;
    pub use super::decrease_bonfire_flame::DecreaseBonfireFlameSystem;
    pub use super::delete_wood_indicator::DeleteWoodIndicatorSystem;
    pub use super::handle_beartrap_affected::HandleBeartrapAffectedSystem;
    pub use super::handle_beartrap_hit::HandleBeartrapHitSystem;
    pub use super::handle_flame_visibility::HandleFlameVisibilitySystem;
    pub use super::handle_ladder_climbing::HandleLadderClimbingSystem;
    pub use super::handle_movables::HandleMovablesSystem;
    pub use super::handle_player_feed_bonfire::HandlePlayerFeedBonfireSystem;
    pub use super::handle_player_wood_pickup::HandlePlayerWoodPickupSystem;
    pub use super::play_random_sounds::PlayRandomSoundsSystem;
    pub use super::song_volume_proximity::SongVolumeProximitySystem;
    pub use super::spawn_beartrap::SpawnBeartrapSystem;
    pub use super::spawn_wood::SpawnWoodSystem;
    pub use super::update_bonfire_halo_size::UpdateBonfireHaloSizeSystem;
    pub use super::update_flame_radius::UpdateFlameRadiusSystem;
    pub use super::update_player_animation::UpdatePlayerAnimationSystem;
    pub use super::update_reactive_animations::UpdateReactiveAnimationsSystem;
    pub use super::update_wood_inventory::UpdateWoodInventorySystem;
    pub use super::update_wood_spawner_manager::UpdateWoodSpawnerManagerSystem;
    pub use deathframe::systems::prelude::*;
}

pub mod system_prelude {
    pub use crate::components::prelude::*;
    pub use crate::input::prelude::*;
    pub use crate::resources::prelude::*;
    pub use crate::settings::prelude::*;
    pub use deathframe::core::geo::prelude::*;
    pub use deathframe::systems::system_prelude::*;
}

mod control_player;
mod decrease_bonfire_flame;
mod delete_wood_indicator;
mod handle_beartrap_affected;
mod handle_beartrap_hit;
mod handle_flame_visibility;
mod handle_ladder_climbing;
mod handle_movables;
mod handle_player_feed_bonfire;
mod handle_player_wood_pickup;
mod play_random_sounds;
mod song_volume_proximity;
mod spawn_beartrap;
mod spawn_wood;
mod update_bonfire_halo_size;
mod update_flame_radius;
mod update_player_animation;
mod update_reactive_animations;
mod update_wood_inventory;
mod update_wood_spawner_manager;
