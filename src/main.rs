mod config;
mod life;
mod systems;

use amethyst::{
    core::transform::TransformBundle,
    prelude::{Application, GameDataBuilder},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use crate::life::Life;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with(systems::CellSystem::default(), "cell_system", &[]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Life::default(), game_data)?;
    game.run();

    Ok(())
}
