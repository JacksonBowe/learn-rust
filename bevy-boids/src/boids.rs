use bevy::prelude::*;

// use rand::prelude::*;

pub struct BoidsPlugin;
impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(boids_system);
    }
}

#[derive(Component, Debug)]
pub struct Boid {
    pub velocity: Vec3,
    pub position: Vec3,
    pub separation: f32,
    pub cohesion: f32,
    pub attraction: f32,
    pub group: Option<String>
}

fn boids_system() {
    println!("Test");
}



/* 
component Boid

struct BoidAttributes
    cohesion
    separation
    alignment
    avoidance
    group

struct verbs
    chasing
    running
    killing
    flocking
    reproducing


struct CreatureAttributes
    color
    speed
    vision
    size
    scare
    chase
    energy



*/