use bevy::prelude::*;

fn create_world() {
    println!("Creating World")
}

pub struct CreateWorldPlugin;

impl Plugin for CreateWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_world);
    }
}