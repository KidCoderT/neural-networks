use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::{shape::Circle, *},
    render::render_graph,
    sprite::MaterialMesh2dBundle,
    window::PresentMode,
};

use new_ui::consts::*;
use new_ui::data;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DotColor {
    Red,
    Blue,
}

#[derive(Component, Debug, Clone)]
struct Dot(DotColor);

impl Dot {
    pub fn u8_color(&self) -> Color {
        match self {
            Dot(DotColor::Red) => Color::rgb_u8(241, 82, 94),
            Dot(DotColor::Blue) => Color::rgb_u8(92, 177, 254),
        }
    }

    pub fn material_color(&self) -> ColorMaterial {
        ColorMaterial::from(self.u8_color())
    }
}

fn initialize_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn create_spots(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (dotx, doty) in &data::BLUES {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(DOT_R).into()).into(),
                material: materials.add(Dot(DotColor::Blue).material_color()),
                transform: Transform::from_translation(Vec3::new(
                    *dotx * TILE_SIZE as f32 + OFFSET as f32,
                (NO_V_TILES as f32 - *doty) * TILE_SIZE as f32 + OFFSET as f32,
                    0.0,
                )),
                ..default()
            },
            Dot(DotColor::Blue),
        ));
    }

    for (dotx, doty) in &data::REDS {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(DOT_R).into()).into(),
                material: materials.add(Dot(DotColor::Red).material_color()),
                transform: Transform::from_translation(Vec3::new(
                    *dotx * TILE_SIZE as f32 + OFFSET as f32,
                (NO_V_TILES as f32 - *doty) * TILE_SIZE as f32 + OFFSET as f32,
                    0.0,
                )),
                ..default()
            },
            Dot(DotColor::Red),
        ));
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(24, 23, 24)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Neural Network - 1".into(),
                resolution: (WIDTH as f32, HEIGHT as f32).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_startup_system(initialize_camera)
        .add_startup_system(create_spots)
        // .add_system(render_graph)
        .run();
}
