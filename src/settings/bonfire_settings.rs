// resources/settings/bonfire.ron

use crate::components::prelude::*;
use crate::resources::prelude::AnimationKey;

#[derive(Clone, Deserialize)]
pub struct BonfireSettings {
    pub size:       Size,
    pub hitbox:     Hitbox,
    pub flame:      Flame,
    pub animations: AnimationsContainer<AnimationKey>,
}