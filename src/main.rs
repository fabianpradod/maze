// src/main.rs
mod caster;
mod framebuffer;
mod input;
mod maze;
mod player;
mod render3d;

use caster::cast_ray;
use framebuffer::Framebuffer;
use input::process_events;
use maze::{load_maze, render_maze};
use player::Player;
use raylib::prelude::*;
use render3d::render3d;

fn main() {
    let maze = load_maze("./maze.txt");

    let block_size = 64;
    let screen_width = 1024;
    let screen_height = 512;

    let (mut rl, thread) =
        raylib::init().size(screen_width as i32, screen_height as i32).title("Maze 3D").build();

    let mut framebuffer = Framebuffer::new(screen_width as u32, screen_height as u32, Color::BLACK);

    let mut player =
        Player::from_maze(&maze, block_size).expect("No se encontró 'p' en el laberinto");

    while !rl.window_should_close() {
        process_events(&rl, &mut player, &maze, block_size);

        framebuffer.clear();
        render3d(&mut framebuffer, &player);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        framebuffer.draw_to_screen(&mut d);
    }
}
