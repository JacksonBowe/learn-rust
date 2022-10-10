use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

/* Rules
    1. Any live cell with two or three live neighbours survives.
    2. Any dead cell with three live neighbours becomes a live cell.
    3. All other live cells die in the next generation. Similarly, all other dead cells stay dead.

    Create a 2D camera

*/ 

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(UpdateTimer(Timer::from_seconds(1.0, true)))
        .add_startup_system(setup_board)
        .add_system(update_board)
        .run();
}

#[derive(Component, Debug)]
struct Board(Vec<Vec<Cell>>);

#[derive(Component, Clone, Debug, Copy)]
struct Cell {
    alive: bool
}

fn setup_board(mut commands: Commands) {
    // Spawn the camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Create the board
    let mut initial = vec![vec![Cell { alive: false} ;10]; 10];
    initial[5][5].alive = true;
    initial[6][6].alive = true;
    initial[6][7].alive = true;
    initial[7][6].alive = true;
    initial[7][7].alive = true;
    initial[8][8].alive = true;

    let board = commands.spawn()
        .insert(Board(initial))
        .id();

    

    println!("{:?}", board);
}

struct UpdateTimer(Timer);


fn update_board(time: Res<Time>, mut timer: ResMut<UpdateTimer>, mut q: Query<&mut Board>) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut board in q.iter_mut() {
            let mut clone = board.0.clone();
            // println!("{:?}", board.0)
            for i in 0..board.0.len() {
                for j in 0..board.0.len() {
                    
                    // Get neighbours
                    let neighbours = [
                        if i > 0                                      { board.0[i-1][j].alive } else { false },     // Left
                        if i > 0 && j > 0                             { board.0[i-1][j-1].alive } else { false },   // Left - Up
                        if j > 0                                      { board.0[i][j-1].alive } else { false },     // Up
                        if i < board.0.len()-1 && j > 0               { board.0[i+1][j-1].alive } else { false },   // Right - Up
                        if i < board.0.len()-1                        { board.0[i+1][j].alive } else { false },     // Right
                        if i < board.0.len()-1 && j < board.0.len()-1 { board.0[i+1][j+1].alive } else { false },   // Right - Down
                        if j < board.0.len()-1                        { board.0[i][j+1].alive } else { false },     // Down
                        if i > 0 && j < board.0.len()-1               { board.0[i-1][j+1].alive } else { false }    // Left - Down
                    ];

                    let alive_neighbours = neighbours.iter().filter(|&n| *n == true).count();
                    if board.0[i][j].alive {
                        println!("Alive: ({},{}) - n={:?}", i, j, &alive_neighbours);
                    }

                    if alive_neighbours < 2 {
                        clone[i][j].alive = false
                    } else if alive_neighbours > 2 {
                        clone[i][j].alive = true
                    } else {
                        clone[i][j].alive = false
                    }
                    
                }
            }
            board.0 = clone.to_vec()
        }
    }
}

fn update_cells(mut commands: Commands, time: Res<Time>, mut timer: ResMut<UpdateTimer>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, mut q: Query<&mut Board>) {
    for mut board
}
