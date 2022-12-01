use bevy::prelude::*;


pub struct CreateWorldPlugin;

impl Plugin for CreateWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_world);
    }
}


fn create_world(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    println!("Creating World");
    // Generate a sign wave
    
    // Step along the sign wave and spawn a cube
    for i in 0..100 {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: Transform::from_xyz(i as f32, i as f32, i as f32),
            ..default()
        });
    }
}