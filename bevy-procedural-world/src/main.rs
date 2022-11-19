use bevy::prelude::*;

mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(world::CreateWorldPlugin)
        // .add_system(hello_world)
        .run();
}

