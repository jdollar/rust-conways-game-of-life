use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BoardConfig {
    pub height: f32,
    pub width: f32,
}

impl Default for BoardConfig {
    fn default() -> Self {
        BoardConfig {
            height: 10.0,
            width: 10.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LifeConfig {
    pub board: BoardConfig,
    pub seed: HashSet<(isize, isize)>,
    pub tick_limit: f32,
}

impl Default for LifeConfig {
  fn default() -> LifeConfig {
    LifeConfig {
      tick_limit: 1.0,
      seed: HashSet::default(),
      board: BoardConfig::default(),
    }
  }
}
