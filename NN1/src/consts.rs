use macroquad::{prelude::*, window};

pub const horizontal_tiles: usize = 14;
pub const vertical_tiles: usize = 8;
pub const tile_size: usize = 70;
pub const skip_by: f32 = 0.1;

pub const width: usize = ((horizontal_tiles + 1) * tile_size);
pub const height: usize = ((vertical_tiles + 1) * tile_size);
pub const offset: usize = tile_size / 2;

pub const circle_radius: f32 = 8.;
