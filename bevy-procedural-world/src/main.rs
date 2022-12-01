use bevy::prelude::*;
use bevy_config_cam::*;

mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(world::CreateWorldPlugin)
        .add_plugin(ConfigCam)
        .add_startup_system(spawn_camera)
        .run();
}


fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
