use bevy::prelude::*;
use rand::prelude::*;

use super::Velocity;

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

fn boids_system(mut boids: Query<(&mut Boid, &mut Transform, &mut Velocity)>) {
    // Calculate aspects for the boid group
    let mut positions = Vec3::ZERO;
    let mut directions = Vec3::ZERO;
    let mut num_boids = 0;
    for (_boid, mut transform, mut velocity) in &mut boids {
        // Add boid position to the positions Vecor
        positions += transform.translation;

        // Add boid direction to the directions Vector
        directions.x += velocity.x;
        directions.y += velocity.y;

        num_boids += 1;

    // Calculate the averages
    let mut average_position = positions / num_boids as f32;
    let mut average_direction = directions / num_boids as f32;

    println!("AvP: {}", average_position);
    println!("AvD: {}", average_direction);

    // Adjust the velocity of each boid based on the averages

    for (_boid, mut transform, mut velocity) in &mut boids {
        velocity += average_position.normalize()
    }
    
        // // Get the velocity vector
        // println!("V: [{}, {}]", velocity.x, velocity.y);
        // // Add boid position to the average

        
        // // Update translation.rotation to match new velocity vector
        // println!("R: {}", transform.rotation);

        // let mut rng = rand::thread_rng();
        // let y: f64 = rng.gen(); // generates a float between 0 and 1
        // let turn_direction = y - 0.5;

        // transform.rotate_z(turn_direction as f32);

        // let test = transform.rotation;
        // transform.translation += test * Vec3::new(1., 1., 0.)
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