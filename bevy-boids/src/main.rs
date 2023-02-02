use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle
};
use boids::Boid;

mod boids;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(boids::BoidsPlugin)
        .add_startup_system(setup)
        .add_system(apply_velocity)
        .run()
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Spawn the camera
    commands.spawn(Camera2dBundle::default());
    commands.spawn((MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::RegularPolygon::new(0.1, 3))).into(),
            transform: Transform::default().with_scale(Vec3::splat(128.)).with_rotation(Quat::default()),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        },
        Boid::default(),
        Velocity(Vec2::new(1.0, 1.0)),
    ));
    commands.spawn((MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::RegularPolygon::new(0.1, 3))).into(),
            transform: Transform::from_xyz(30., 0., 0.).with_scale(Vec3::splat(128.)).with_rotation(Quat::default()),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        },
        Boid::default(),
        Velocity(Vec2::new(1.0, 1.0)),
    ));
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    let dt = time.delta_seconds();
    let speed = 100.;
    for (mut transform, velocity) in &mut query {
        // println!("{}", transform.translation);
        transform.translation.x += velocity.x * speed * dt;
        transform.translation.y += velocity.y * speed * dt;
    }
}

// fn pointer_system(mut query: Query<&Transform>, mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterials>>) {
//     for entity in &mut query {
//         println!("{}", entity)
//     }
// }