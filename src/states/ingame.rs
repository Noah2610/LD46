use super::state_prelude::*;
use crate::level_loader;

pub struct Ingame {
    level_name: String,
}

impl Ingame {
    pub fn new(level_name: String) -> Self {
        Self { level_name }
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        data.world.delete_all();

        level_loader::load_level(
            resource(format!("levels/{}", &self.level_name)),
            data.world,
        )
        .unwrap();
    }

    fn on_stop(&mut self, data: StateData<GameData<'a, 'b>>) {
        data.world.delete_all();
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, DispatcherId::Ingame).unwrap();

        let input_manager =
            data.world.read_resource::<InputManager<MenuBindings>>();
        if input_manager.is_down(MenuAction::Back) {
            return Trans::Pop;
        }

        Trans::None
    }
}
