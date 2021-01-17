mod snake;
mod config;

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use crate::config::ArenaConfig;


use crate::snake::Snake;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("config").join("display.ron");
    let config = app_root.join("config").join("config.ron");
    let arena_config = ArenaConfig::load(config)?;
    println!("{:?}", arena_config);

    let game_data = GameDataBuilder::default().with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            .with_plugin(RenderFlat2D::default()),
    )?;

    let assets_dir = app_root.join("assets");

    let mut game = Application::new(assets_dir, Snake, game_data)?;

    game.run();

    Ok(())
}
