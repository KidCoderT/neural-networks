use macroquad::{prelude::*};

extern crate nn1;
use nn1::consts::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Data Gen".to_owned(),
        window_resizable: false,
        fullscreen: false,
        window_width: ((NO_H_TILES + 1) * TILE_SIZE) as i32,
        window_height: ((NO_V_TILES + 1) * TILE_SIZE) as i32,
        ..Default::default()
    }
}

enum Side {
    Blue,
    Red,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut reds: Vec<(f32, f32)> = vec![];
    let mut blues: Vec<(f32, f32)> = vec![];

    let background = Color::from_rgba(24, 23, 24, 255);
    let graph_lines = Color::from_rgba(39, 38, 39, 255);
    let highlighted = Color::from_rgba(88, 86, 88, 255);
    let red_color = Color::from_rgba(241, 82, 94, 255);
    let blue_color = Color::from_rgba(92, 177, 254, 255);

    let mut active_color: Side = Side::Blue;

    loop {
        clear_background(background);

        if is_key_down(KeyCode::Q) || is_key_down(KeyCode::Escape) {
            break;
        }

        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) || is_key_down(KeyCode::Left) {
            active_color = match active_color {
                Side::Blue => Side::Red,
                Side::Red => Side::Blue,
            };
        }

        for idx in 0..=NO_H_TILES {
            let x = (idx * TILE_SIZE + OFFSET) as f32;
            let line_color = if idx == 0 { highlighted } else { graph_lines };
            draw_line(x, 0., x, HEIGHT as f32, 2., line_color)
        }

        for idx in 0..=NO_V_TILES {
            let y = (idx * TILE_SIZE + OFFSET) as f32;
            let line_color = if idx == NO_V_TILES {
                highlighted
            } else {
                graph_lines
            };
            draw_line(0., y, WIDTH as f32, y, 2., line_color)
        }

        for (idx, (dotx, doty)) in blues.clone().iter().enumerate() {
            let x = *dotx * TILE_SIZE as f32 + OFFSET as f32;
            let y = *doty * TILE_SIZE as f32 + OFFSET as f32;

            draw_circle(x, y, DOT_R, blue_color);

            draw_text(format!("{}", &idx).as_str(), x, y, 22., WHITE);
        }

        for (idx, (dotx, doty)) in reds.clone().iter().enumerate() {
            let x = *dotx * TILE_SIZE as f32 + OFFSET as f32;
            let y = *doty * TILE_SIZE as f32 + OFFSET as f32;

            draw_circle(x, y, DOT_R, red_color);

            draw_text(format!("{}", &idx).as_str(), x, y, 22., WHITE);
        }

        if is_mouse_button_released(MouseButton::Left) {
            let (x, y) = {
                let positions = mouse_position();
                (
                    (positions.0 - OFFSET as f32) / TILE_SIZE as f32,
                    (positions.1 - OFFSET as f32) / TILE_SIZE as f32,
                )
            };
            let vector = ((x * 100.0).round() / 100.0, (y * 100.0).round() / 100.0);

            match active_color {
                Side::Blue => {
                    blues.push(vector);
                }
                Side::Red => {
                    reds.push(vector);
                }
            }
        }

        next_frame().await;
    }

    println!("Reds: {:?}", reds);
    println!("Blues: {:?}", blues);
}
