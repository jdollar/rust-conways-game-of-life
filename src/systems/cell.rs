use std::collections::HashMap;

use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::{ParJoin, Join, Read, System, SystemData, WriteStorage},
    renderer::{palette::Srgba, resources::Tint},
};

use rayon::prelude::*;
use crate::life::Cell;
use crate::config::LifeConfig;

#[derive(SystemDesc, Default)]
pub struct CellSystem {
    last_update_time_secs: f32,
}

impl<'s> System<'s> for CellSystem {
    type SystemData = (
        WriteStorage<'s, Cell>,
        WriteStorage<'s, Tint>,
        Read<'s, Time>,
        Read<'s, LifeConfig>,
    );

    fn run(&mut self, (mut cells, mut tints, time, config): Self::SystemData) {
        // Stopping it from running until we hit a set "tick rate" in seconds
        // according to a config value
        self.last_update_time_secs += time.delta_seconds();
        if self.last_update_time_secs < config.tick_limit { return; }
        self.last_update_time_secs = 0.0;

        // Grab the information on if a cell is alive at specific positions on the board
        // Stores them in a hashmap with a key of "column:row" and a value of the is_alive boolean
        // Maybe this should be somewhere else? Resource on world? Would need to benchmark if the
        // variable recreation is a bottleneck vs updating an existing refrence
        let mut board_cell_status: HashMap<String, bool> = HashMap::new();
        for cell in (&cells).join() {
            if let Some(column) = cell.board_column {
                if let Some(row) = cell.board_row {
                    board_cell_status.insert(format!("{}:{}", column, row), cell.is_alive);
                }
            }
        }

        // Doing par_join here, mainly to try it out. Didn't really do any
        // benchmarking to see if there is any benefit/detriment. Probably
        // depends on the board size configured in the config.ron
        (&mut cells, &mut tints).par_join().for_each(|(cell, tint)| {
            if let Some(column) = cell.board_column {
                if let Some(row) = cell.board_row {
                    // Determines what keys in the board_cell_status hashmap are considered
                    // neighbors to the current cell. Does create some negative numbers
                    // and numbers out of range for the board that is created for the edge
                    // cells, but since they aren't in the board_cell_status map they just
                    // won't be counted as alive
                    let neighbor_keys = [
                        format!("{}:{}", column - 1, row - 1), // top left
                        format!("{}:{}", column - 1, row),     // left
                        format!("{}:{}", column - 1, row + 1), // bottom left
                        format!("{}:{}", column, row - 1),     // top
                        format!("{}:{}", column, row + 1),     // bottom
                        format!("{}:{}", column + 1, row - 1), // top right
                        format!("{}:{}", column + 1, row),     // right
                        format!("{}:{}", column + 1, row + 1), // bottom right
                    ];

                    // Determines what of it's neighbor's are alive and keeps track of the
                    // number of it's alive neighbors in this variable here
                    let mut alive_neighbors = 0;
                    for neighbor_key in neighbor_keys.iter() {
                        if let Some(neighbor_is_alive) = board_cell_status.get(neighbor_key) {
                            if *neighbor_is_alive {
                                alive_neighbors += 1;
                            }
                        }
                    }

                    // Uses the alive_neighbors varaible to determine the game rules:
                    // Any live cell with two or three live neighbours survives.
                    // Any dead cell with three live neighbours becomes a live cell.
                    // All other live cells die in the next generation. Similarly, all other dead cells stay dead.
                    cell.is_alive = match alive_neighbors {
                        3 => true,
                        2 if cell.is_alive => true,
                        _ => false,
                    };

                    // Sets the color to black (alive) or white (dead) based on the new cell status
                    tint.0 = match cell.is_alive {
                        false => Srgba::new(1.0, 1.0, 1.0, 1.0),
                        true => Srgba::new(0.0, 0.0, 0.0, 1.0),
                    };
                }
            }
        });
    }
}
