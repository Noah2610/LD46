mod bonfire;
mod camera;
mod player;
mod wood;

mod init_prelude {
    pub use crate::components::prelude::*;
    pub use crate::resource;
    pub use crate::resources::prelude::*;
    pub use crate::settings::prelude::*;
    pub use amethyst::ecs::{Entity, World, WorldExt};
    pub use amethyst::prelude::Builder;
    pub use deathframe::amethyst;
    pub use std::path::PathBuf;
}

pub use bonfire::init_bonfire;
pub use camera::init_camera;
pub use player::init_player;
pub use wood::init_wood;
