use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle
};

mod boids;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(boids::BoidsPlugin)
        .add_startup_system(setup)
        .run()
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Spawn the camera
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        // mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        mesh: meshes.add(Mesh::from(shape::RegularPolygon::new(0.1, 3))).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    });
}