use crate::ArenaConfig;
use amethyst::{GameData, SimpleState, StateData};
use amethyst::utils::application_root_dir;
use amethyst::prelude::WorldExt;

pub struct Snake;

impl SimpleState for Snake {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let (arena_height, arena_width) = {
            let config = &world.read_resource::<ArenaConfig>();
            (config.height, config.width)
        };
    }
}
