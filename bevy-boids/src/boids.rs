use bevy::prelude::*;

use rand::prelude::*;

pub struct BoidsPlugin;
impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(boids_system);
    }
}

#[derive(Component, Debug)]
pub struct Boid {
    pub velocity: Vec2,
    pub position: Vec2,
    pub separation: f32,
    pub cohesion: f32,
    pub attraction: f32,
    pub group: i32
}

impl Default for Boid {
    fn default() -> Self {
        Boid {
            velocity: Vec2::ZERO,
            position: Vec2::ZERO,
            separation: 0.1,
            cohesion: 0.1,
            attraction: 0.1,
            group: 0
        }
    }
}

fn boids_system(mut boids: Query<(&mut Boid, &mut Transform)>) {
    // Calculate aspects for the boid group
    for (_boid, mut transform) in &mut boids {
        println!("{}", transform.rotation);

        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen(); // generates a float between 0 and 1
        let turn_direction = y - 0.5;

        transform.rotate_z(turn_direction as f32);

        let test = transform.rotation;
        transform.translation += test * Vec3::splat(1.)
    }
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