pub mod prelude {
    pub use super::control_player::ControlPlayerSystem;
    pub use super::handle_movables::HandleMovablesSystem;
    pub use deathframe::systems::prelude::*;
}

mod system_prelude {
    pub use crate::components::prelude::*;
    pub use crate::input::prelude::*;
    pub use crate::resources::prelude::*;
    pub use crate::settings::prelude::*;
    pub use deathframe::core::geo::prelude::*;
    pub use deathframe::systems::system_prelude::*;
}

mod control_player;
mod handle_movables;
