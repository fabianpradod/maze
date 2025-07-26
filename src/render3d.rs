// src/render3d.rs
use crate::caster::cast_ray;
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::Player;
use raylib::prelude::*;

pub fn render3d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let block_size = 64;
    let num_rays = framebuffer.width;

    let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;

    framebuffer.set_current_color(Color::WHITE);

    for i in (0..num_rays).step_by(8) {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, player, a, block_size, false);

        if intersect.distance < 1.0 {
            continue;
        }

        let distance_to_wall = intersect.distance;
        let distance_to_projection_plane = hw;
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

        let stake_height = stake_height.min(framebuffer.height as f32);

        let stake_top = ((hh - (stake_height / 2.0)).max(0.0)) as usize;
        let stake_bottom = ((hh + (stake_height / 2.0)).min(framebuffer.height as f32)) as usize;

        for y in stake_top..stake_bottom {
            if y < framebuffer.height as usize {
                framebuffer.set_pixel(i, y as u32);
            }
        }
    }
}
