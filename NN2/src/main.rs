use std::time::Instant;

use macroquad::prelude::*;

use macroquad::ui::{
    hash, root_ui,
    widgets::{self},
};

use nn2::consts::*;
use nn2::data;
use nn2::nn::NeuralNetwork;

fn window_conf() -> Conf {
    Conf {
        window_title: "Neural Network - 1".to_owned(),
        window_resizable: false,
        fullscreen: false,
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let background = Color::from_rgba(24, 23, 24, 255);
    let graph_lines = Color::from_rgba(39, 38, 39, 255);
    let highlighted = Color::from_rgba(88, 86, 88, 255);
    let red_color = Color::from_rgba(241, 82, 94, 255);
    let red_tint = Color::from_rgba(241, 82, 94, 100);
    let blue_color = Color::from_rgba(92, 177, 254, 255);
    let blue_tint = Color::from_rgba(92, 177, 254, 100);

    let mut texture = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, red_tint);
    let mut network = NeuralNetwork::new_network(&[2, 3, 2]);

    let time = Instant::now();
    for x in -(OFFSET as i16)..(WIDTH - OFFSET) as i16 {
        // println!("pixel x: {}, graph x: {}", x + OFFSET as i16, x);
        for y in (0i16..HEIGHT as i16).rev() {
            // println!("pixel y: {}, graph y: {}", y);
            let prediction = network.predict(vec![
                x as f32 / WIDTH as f32,
                (HEIGHT as f32 - y as f32 - OFFSET as f32) / HEIGHT as f32,
            ]);
            let color = if prediction == 0 { blue_tint } else { red_tint };
            texture.set_pixel((x + OFFSET as i16) as u32, y as u32, color);
        }
    }
    let duration = time.elapsed();
    println!("duration for command: {:?}", duration);

    loop {
        clear_background(background);

        if is_key_down(KeyCode::Q) || is_key_down(KeyCode::Escape) {
            break;
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

        draw_texture(Texture2D::from_image(&texture), 0.0, 0.0, WHITE);

        for (dotx, doty) in &data::BLUES {
            draw_circle(
                *dotx * TILE_SIZE as f32 + OFFSET as f32,
                *doty * TILE_SIZE as f32 + OFFSET as f32,
                DOT_R,
                blue_color,
            )
        }

        for (dotx, doty) in &data::REDS {
            draw_circle(
                *dotx * TILE_SIZE as f32 + OFFSET as f32,
                *doty * TILE_SIZE as f32 + OFFSET as f32,
                DOT_R,
                red_color,
            )
        }

        widgets::Window::new(hash!(), vec2(WIDTH as f32 - 350., 50.), vec2(300., 150.)).ui(
            &mut *root_ui(),
            |ui| {
                ui.separator();

                for (idx, layer) in network.layers.iter_mut().enumerate() {
                    ui.tree_node(hash!(idx, &layer.name), &layer.name, |ui| {
                        for node_idx in 0..(layer.outputs) {
                            ui.label(None, format!("node: {}", node_idx).as_str());

                            for input_node in (0..layer.inputs) {
                                ui.slider(
                                    hash!((input_node, node_idx, &layer.name)),
                                    format!("[w({},{})]", input_node, node_idx).as_str(),
                                    -1f32..1f32,
                                    &mut layer.weights[input_node][node_idx],
                                );
                            }

                            ui.slider(
                                hash!((node_idx, &layer.name)),
                                format!("[b({})]", node_idx).as_str(),
                                -1f32..1f32,
                                &mut layer.biases[node_idx],
                            );
                        }
                    });
                }

                ui.separator();
            },
        );

        next_frame().await;
    }
}
