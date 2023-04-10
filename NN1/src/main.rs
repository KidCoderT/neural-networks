use macroquad::prelude::*;

use macroquad::ui::{
    hash, root_ui,
    widgets::{self},
};

use nn1::consts::*;
use nn1::data;

fn window_conf() -> Conf {
    Conf {
        window_title: "Neural Network - 1".to_owned(),
        window_resizable: false,
        fullscreen: false,
        window_width: width as i32,
        window_height: height as i32,
        ..Default::default()
    }
}

fn classify(x: f32, y: f32, weights: [[f32; 2]; 2], biases: [f32; 2]) -> u8 {
    let is_okay = x * weights[0][0] + y * weights[1][0] + biases[0];
    let not_okay = x * weights[0][1] + y * weights[1][1] + biases[1];

    // 0 is blue
    // 1 is red

    return if (is_okay > not_okay) { 0 } else { 1 };
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut weights: [[f32; 2]; 2] = [[0f32, 0f32], [0f32, 0f32]];
    let mut biases: [f32; 2] = [0f32, 0f32];

    let background = Color::from_rgba(24, 23, 24, 255);
    let graph_lines = Color::from_rgba(39, 38, 39, 255);
    let highlighted = Color::from_rgba(88, 86, 88, 255);
    let red_color = Color::from_rgba(241, 82, 94, 255);
    let red_tint = Color::from_rgba(241, 82, 94, 100);
    let blue_color = Color::from_rgba(92, 177, 254, 255);
    let blue_tint = Color::from_rgba(92, 177, 254, 100);

    let mut texture = Image::gen_image_color(width as u16, height as u16, red_tint);

    loop {
        clear_background(background);

        if is_key_down(KeyCode::Q) || is_key_down(KeyCode::Escape) {
            break;
        }

        for idx in 0..=horizontal_tiles {
            let x = (idx * tile_size + offset) as f32;
            let line_color = if (idx == 0) { highlighted } else { graph_lines };
            draw_line(x, 0., x, height as f32, 2., line_color)
        }

        for idx in 0..=vertical_tiles {
            let y = (idx * tile_size + offset) as f32;
            let line_color = if (idx == vertical_tiles) {
                highlighted
            } else {
                graph_lines
            };
            draw_line(0., y as f32, width as f32, y as f32, 2., line_color)
        }

        for x in -(offset as i16)..(width - offset) as i16 {
            // println!("pixel x: {}, graph x: {}", x + offset as i16, x);

            for y in (0i16..height as i16).rev() {

                // println!("pixel y: {}, graph y: {}", y);
                let prediction = classify(
                    x as f32,
                    height as f32 - y as f32 - offset as f32,
                    weights,
                    biases,
                );
                let color = if prediction == 0u8 {
                    blue_tint
                } else {
                    red_tint
                };
                texture.set_pixel((x + offset as i16) as u32, y as u32, color);

            }

        }

        draw_texture(Texture2D::from_image(&texture), 0.0, 0.0, WHITE);

        for (dotx, doty) in &data::BLUES {
            draw_circle(
                *dotx * tile_size as f32 + offset as f32,
                *doty * tile_size as f32 + offset as f32,
                circle_radius,
                blue_color,
            )
        }

        for (dotx, doty) in &data::REDS {
            draw_circle(
                *dotx * tile_size as f32 + offset as f32,
                *doty * tile_size as f32 + offset as f32,
                circle_radius,
                red_color,
            )
        }

        widgets::Window::new(hash!(), vec2(width as f32 - 350., 50.), vec2(300., 150.)).ui(
            &mut *root_ui(),
            |ui| {
                ui.separator();

                ui.tree_node(hash!(), "weights", |ui| {
                    for layer_idx in 0..(weights.len()) {
                        for node_idx in 0..(weights[layer_idx].len()) {
                            ui.slider(
                                hash!((layer_idx, node_idx)),
                                format!("[w({},{})]", layer_idx, node_idx).as_str(),
                                -1f32..1f32,
                                &mut weights[layer_idx][node_idx],
                            );
                        }
                    }
                });

                ui.separator();

                ui.tree_node(hash!(), "biases", |ui| {
                    for node_bias_idx in 0..(biases.len()) {
                        ui.slider(
                            hash!((node_bias_idx,)),
                            format!("[b({})]", node_bias_idx).as_str(),
                            -1f32..1f32,
                            &mut biases[node_bias_idx],
                        );
                    }
                });
            },
        );

        next_frame().await;
    }
}
