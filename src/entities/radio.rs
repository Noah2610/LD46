use super::init_prelude::*;

pub fn init_radio(world: &mut World, transform: Transform) -> Entity {
    let radio_settings = world.read_resource::<Settings>().radio.clone();

    let mut radio_builder = world
        .create_entity()
        .with(Radio::default())
        .with(transform)
        .with(ScaleOnce::default())
        .with(Transparent)
        .with(VisibleInFlame::default());

    radio_builder.build()
}
