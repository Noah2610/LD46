use super::state_prelude::*;
use crate::resource;
use std::path::PathBuf;

#[derive(Default)]
pub struct Startup;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Startup {
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        #[cfg(not(feature = "debug"))]
        enter_fullscreen(data.world);

        data.world.register::<crate::components::prelude::Radio>();

        insert_resources(data.world);
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update_core(data.world);
        Trans::Switch(Box::new(MainMenu::default()))
    }
}

fn insert_resources(world: &mut World) {
    let settings = Settings::load().unwrap();
    world.insert(settings);

    let mut sprite_sheet_handles = SpriteSheetHandles::<PathBuf>::default();
    sprite_sheet_handles.load(resource("spritesheets/wood.png"), world);
    sprite_sheet_handles
        .load(resource("spritesheets/wood_indicator.png"), world);
    world.insert(sprite_sheet_handles);

    load_songs(world);
    load_sounds(world);
}

fn load_songs(world: &mut World) {
    let songs_settings = world.read_resource::<Settings>().songs.clone();

    let mut songs = Songs::<SongKey>::default();

    for (song_key, song_settings) in songs_settings.songs {
        songs
            .load_audio(
                song_key,
                resource(format!("audio/bgm/{}", song_settings.file)),
                song_settings.should_loop,
                world,
            )
            .unwrap();
    }

    world.insert(songs);
}

fn load_sounds(world: &mut World) {
    use crate::components::prelude::SoundPlayer;

    world.insert(SoundPlayer::<SoundKey>::default());

    let sounds_settings = world.read_resource::<Settings>().sounds.clone();

    let mut sounds = Sounds::<SoundKey>::default();
    sounds
        .load_audio(
            "beartrap".to_string(),
            resource("audio/sfx/beartrap.wav"),
            world,
        )
        .unwrap();
    sounds
        .load_audio(
            "play_with_headphones".to_string(),
            resource("audio/sfx/play_with_headphones.wav"),
            world,
        )
        .unwrap();
    sounds
        .load_audio(
            "woodblock".to_string(),
            resource("audio/sfx/woodblock.wav"),
            world,
        )
        .unwrap();

    for sound_group in sounds_settings.sound_groups {
        for sound in sound_group.sounds {
            let sound_filepath = resource(format!("audio/sfx/{}", sound.key));
            sounds.load_audio(sound.key, sound_filepath, world).unwrap();
        }
    }

    world.insert(sounds);
}

fn enter_fullscreen(world: &mut World) {
    use amethyst::ecs::{ReadExpect, SystemData};
    use amethyst::renderer::rendy::wsi::winit::Window;
    use amethyst::window::MonitorIdent;
    use deathframe::amethyst;

    // let window = world.read_resource::<Window>();
    let window = <ReadExpect<'_, Window>>::fetch(world);
    let monitor_ident = MonitorIdent::from_primary(&*window);
    let monitor_id = monitor_ident.monitor_id(&*window);

    window.set_fullscreen(Some(monitor_id));
}
