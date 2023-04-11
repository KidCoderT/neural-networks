use macroquad::prelude::*;

pub const NO_H_TILES: usize = 14;
pub const NO_V_TILES: usize = 8;
pub const TILE_SIZE: usize = 70;

pub const WIDTH: usize = (NO_H_TILES + 1) * TILE_SIZE;
pub const HEIGHT: usize = (NO_V_TILES + 1) * TILE_SIZE;
pub const OFFSET: usize = TILE_SIZE / 2;

pub const DOT_R: f32 = 8.;

pub fn max_item_idx(vector: Vec<f32>) -> usize {
    let mut max_index = 0;
    let mut max_value = vector[0];

    for (index, &value) in vector.iter().enumerate().skip(1) {
        if value > max_value {
            max_index = index;
            max_value = value;
        }
    }

    return max_index;
}
