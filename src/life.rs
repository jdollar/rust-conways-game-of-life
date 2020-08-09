use rand::Rng;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    prelude::{
        Builder, Config, GameData, SimpleState, SimpleTrans, StateData, Trans, World, WorldExt,
    },
    renderer::{
        palette::Srgba, resources::Tint, Camera, ImageFormat, SpriteRender, SpriteSheet,
        SpriteSheetFormat, Texture,
    },
    utils::application_root_dir,
};

use crate::config::LifeConfig;
use specs::{Component, DenseVecStorage};

// Interested if there might be a clean way to pull these values
// off the sprite configuration. SHOULD I make these values
// based off something in the sprite config?
pub const CELL_HEIGHT: f32 = 4.0;
pub const CELL_WIDTH: f32 = 4.0;

#[derive(Default)]
pub struct Life {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Cell {
    pub is_alive: bool,
    pub board_column: Option<isize>,
    pub board_row: Option<isize>,
}

impl Cell {
    fn new(column: isize, row: isize, is_alive: bool) -> Cell {
        Cell {
            is_alive: is_alive,
            board_column: Some(column),
            board_row: Some(row),
        }
    }
}

fn get_board_dimensions(world: &World) -> (f32, f32) {
    let config = world.read_resource::<LifeConfig>();
    (config.board.height, config.board.width)
}

fn initialize_config(world: &mut World) {
    let app_root = application_root_dir().unwrap(); // TODO: Cleaner way to unwrap
    let config_path = app_root.join("config").join("config.ron");
    let config = LifeConfig::load(&config_path).unwrap(); // TODO: Cleaner way to unwrap

    world.insert(config);
}

fn initialize_camera(world: &mut World) {
    let (board_height, board_width) = get_board_dimensions(world);
    let board_pixel_height = board_height * CELL_HEIGHT;
    let board_pixel_width = board_width * CELL_WIDTH;

    let mut transform = Transform::default();
    transform.set_translation_xyz(board_pixel_width * 0.5, board_pixel_height * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(board_pixel_width, board_pixel_height))
        .with(transform)
        .build();
}

fn initialize_cells(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut rng = rand::thread_rng();
    let cell_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    let (board_height, board_width) = get_board_dimensions(world);
    // Used if you want to test out seeds pulled from the config file
    // let seed = {
    //   let config = world.read_resource::<LifeConfig>();
    //   config.seed.clone()
    // };

    for i in 0..board_width.floor() as isize {
        for j in 0..board_height.floor() as isize {
            let mut transform = Transform::default();

            // Determines the anchor points in the x and y coordinates
            // that would be a middle value for the sprite based on it's
            // location on the board
            let cell_x_pos = (i as f32 * CELL_WIDTH) + (CELL_WIDTH * 0.5);
            let cell_y_pos = (j as f32 * CELL_HEIGHT) + (CELL_HEIGHT * 0.5);
            transform.set_translation_xyz(cell_x_pos, cell_y_pos, 0.0);

            // Just randomly selecting if it is alive or not. Improvement
            // may be allowing for a easy way of seeding in the future
            let is_alive = rng.gen_range(0, 2) == 1;

            // Used if you want to test out seeds pulled from the config file
            // let is_alive = if seed.contains(&(i, j)) { true } else { false };

            // black for live cells and white for live ones
            let tint = match is_alive {
              true => Tint(Srgba::new(0.0, 0.0, 0.0, 1.0)),
              false => Tint(Srgba::new(1.0, 1.0, 1.0, 1.0)),
            };

            world
                .create_entity()
                .with(cell_render.clone())
                .with(Cell::new(i, j, is_alive))
                .with(transform)
                .with(tint)
                .build();
        }
    }
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();

    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();

        loader.load(
            "textures/sprite_sheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/sprite_sheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

impl SimpleState for Life {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.sprite_sheet_handle.replace(load_sprite_sheet(world));

        initialize_config(world);
        initialize_camera(world);

        // I guess unwrap is cool here. If we don't have any sprite handles we probably
        // just want to crash
        initialize_cells(world, self.sprite_sheet_handle.clone().unwrap());
    }

    fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}
