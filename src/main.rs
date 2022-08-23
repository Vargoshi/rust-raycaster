#![deny(clippy::all)]
#![forbid(unsafe_code)]

use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub mod rgb_texture_data;
use crate::rgb_texture_data::RGB_TEXTURES;

pub mod sky;
use crate::sky::SKY_DATA;

pub mod title;
use crate::title::TITLE;

pub mod won;
use crate::won::WON;

pub mod lost;
use crate::lost::LOST;

pub mod sprites;
use crate::sprites::SPRITES;

use std::time::Instant;

const SCREEN_WIDTH: u32 = 120;
const SCREEN_HEIGHT: u32 = 80;

use std::f32::consts::PI;

const P2: f32 = PI / 2.0;
const P3: f32 = 3.0 * PI / 2.0;
const DR: f32 = 0.017_453_3;
const TILE_SIZE: usize = 64;

struct Map {
    width: i32,
    height: i32,
    wall_tiles: Vec<i32>,
    floor_tiles: Vec<i32>,
    ceiling_tiles: Vec<i32>,
}

struct Player {
    x: f32,
    y: f32,
    angle: f32,
}

struct Sprite {
    npc_type: i32,
    state: i32,
    map: i32,
    x: f32,
    y: f32,
    z: f32,
}

struct Keyboard {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

fn dist(ax: f32, ay: f32, bx: f32, by: f32, _ang: f32) -> f32 {
    ((bx - ax) * (bx - ax) + (by - ay) * (by - ay)).sqrt()
}

fn draw_pixel(red: u8, green: u8, blue: u8, x: i32, y: i32, frame: &mut [u8]) {
    frame[((y * SCREEN_WIDTH as i32 + x) as usize * 4)] = red; //red
    frame[((y * SCREEN_WIDTH as i32 + x) as usize * 4) + 1] = green; //green
    frame[((y * SCREEN_WIDTH as i32 + x) as usize * 4) + 2] = blue; //blue
    frame[((y * SCREEN_WIDTH as i32 + x) as usize * 4) + 3] = 255; //alpha
}

fn main() -> Result<(), String> {
    let mut map1 = Map {
        width: 8,
        height: 8,
        wall_tiles: vec![
            1, 1, 1, 1, 1, 3, 1, 1, 6, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 4, 0, 2, 0, 1, 1, 5, 4, 5, 0,
            0, 0, 1, 2, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 0, 0, 1, 0, 1, 2, 0, 0, 0, 0, 0, 0, 1, 1, 1,
            3, 1, 3, 1, 3, 1,
        ],
        floor_tiles: vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 2, 0, 1, 0, 0, 0, 0, 0, 1,
            1, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ],
        ceiling_tiles: vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 4, 2, 4, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ],
    };

    let mut player1 = Player {
        x: 300.0,
        y: 300.0,
        angle: 0.0,
    };

    let mut sprite1 = Sprite {
        npc_type: 1,
        state: 1,
        map: 0,
        x: 2.0 * 64.0,
        y: 6.0 * 64.0,
        z: 20.0,
    };
    let mut sprite2 = Sprite {
        npc_type: 2,
        state: 1,
        map: 1,
        x: 1.5 * 64.0,
        y: 4.5 * 64.0,
        z: 1.0,
    };
    let mut sprite3 = Sprite {
        npc_type: 2,
        state: 1,
        map: 1,
        x: 3.5 * 64.0,
        y: 4.5 * 64.0,
        z: 1.0,
    };
    let mut sprite4 = Sprite {
        npc_type: 3,
        state: 1,
        map: 2,
        x: 2.5 * 64.0,
        y: 2.0 * 64.0,
        z: 20.0,
    };

    let mut depth: [i32; 120] = [0; 120];

    let mut keys = Keyboard {
        up: false,
        down: false,
        left: false,
        right: false,
    };

    let time = Instant::now();
    let mut frame1 = 0;

    let mut game_state = 0;
    let mut timer = 0;
    let mut fade = 0.0;

    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(640, 400);
        WindowBuilder::new()
            .with_title("Raycaster")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            if input.key_pressed(VirtualKeyCode::Left) {
                keys.left = true;
            }

            if input.key_pressed(VirtualKeyCode::Right) {
                keys.right = true;
            }

            if input.key_pressed(VirtualKeyCode::Up) {
                keys.up = true;
            }

            if input.key_pressed(VirtualKeyCode::Down) {
                keys.down = true;
            }

            if input.key_pressed(VirtualKeyCode::E) {
                door_open(&player1, &mut map1, &mut sprite1);
            }

            if input.key_released(VirtualKeyCode::Left) {
                keys.left = false;
            }

            if input.key_released(VirtualKeyCode::Right) {
                keys.right = false;
            }

            if input.key_released(VirtualKeyCode::Up) {
                keys.up = false;
            }

            if input.key_released(VirtualKeyCode::Down) {
                keys.down = false;
            }

            let frame2 = time.elapsed().as_millis();
            let fps = frame2 - frame1;
            frame1 = time.elapsed().as_millis();

            if game_state == 0 {
                fade = 0.0;
                timer = 0;
                game_state = 1;
            }
            if game_state == 1 {
                if fade < 1.0 {
                    fade += 0.0005 * (fps) as f32;
                }
                screen(1, fade, pixels.get_frame());
                timer += fps;
                if timer > 3000 {
                    timer = 0;
                    game_state = 2;
                }

                player1.x = 300.0;
                player1.y = 300.0;
                player1.angle = 0.0;
                sprite4.x = 2.5 * 64.0;
                sprite4.y = 2.0 * 64.0;
                sprite1.state = 1;
                map1.wall_tiles[19] = 4;
                map1.wall_tiles[26] = 4;
            }
            if game_state == 2 {
                keyboard_input(&keys, &mut player1, fps, &map1);

                draw_sky(&player1, pixels.get_frame());
                draw_rays(&player1, &map1, pixels.get_frame(), &mut depth);

                if (player1.x as i32 >> 6) == 1 && (player1.y as i32 >> 6) == 1 {
                    fade = 0.0;
                    timer = 0;
                    game_state = 3;
                }

                draw_sprite(
                    &mut sprite1,
                    &player1,
                    depth,
                    &mut game_state,
                    fps,
                    &map1,
                    pixels.get_frame(),
                );
                draw_sprite(
                    &mut sprite2,
                    &player1,
                    depth,
                    &mut game_state,
                    fps,
                    &map1,
                    pixels.get_frame(),
                );
                draw_sprite(
                    &mut sprite3,
                    &player1,
                    depth,
                    &mut game_state,
                    fps,
                    &map1,
                    pixels.get_frame(),
                );
                draw_sprite(
                    &mut sprite4,
                    &player1,
                    depth,
                    &mut game_state,
                    fps,
                    &map1,
                    pixels.get_frame(),
                );
            }

            if game_state == 3 {
                if fade < 1.0 {
                    fade += 0.0005 * (fps) as f32;
                }
                screen(2, fade, pixels.get_frame());
                timer += fps;
                if timer > 3000 {
                    fade = 0.0;
                    timer = 0;
                    game_state = 0;
                }
            }

            if game_state == 4 {
                screen(3, fade, pixels.get_frame());
                timer += fps;
                if timer > 3000 {
                    fade = 0.0;
                    timer = 0;
                    game_state = 0;
                }
            }

            window.request_redraw();
        }
    });
}

fn door_open(player: &Player, map1: &mut Map, sprite: &mut Sprite) {
    if sprite.state == 0 {
        let x_offset = if player.angle.cos() < 0.0 { -25 } else { 25 };

        let y_offset = if player.angle.sin() < 0.0 { -25 } else { 25 };

        let ipx_add_xo = (player.x as i32 + x_offset) / 64;
        let ipy_add_yo = (player.y as i32 + y_offset) / 64;
        if map1.wall_tiles[(ipy_add_yo * map1.width + ipx_add_xo) as usize] == 4 {
            map1.wall_tiles[(ipy_add_yo * map1.width + ipx_add_xo) as usize] = 0;
        }
    }
}

fn keyboard_input(keys: &Keyboard, player: &mut Player, fps: u128, map: &Map) {
    let x_offset = if player.angle.cos() < 0.0 { -20 } else { 20 };

    let y_offset = if player.angle.sin() < 0.0 { -20 } else { 20 };

    let ipx = player.x / 64.0;
    let ipx_add_xo = (player.x as i32 + x_offset) / 64;
    let ipx_sub_xo = (player.x as i32 - x_offset) / 64;
    let ipy = player.y / 64.0;
    let ipy_add_yo = (player.y as i32 + y_offset) / 64;
    let ipy_sub_yo = (player.y as i32 - y_offset) / 64;

    if keys.up {
        // move the player forward.

        if map.wall_tiles[(ipy as i32 * map.width + ipx_add_xo) as usize] == 0 {
            player.x += player.angle.cos() * 0.2 * fps as f32;
        }

        if map.wall_tiles[(ipy_add_yo * map.width + ipx as i32) as usize] == 0 {
            player.y += player.angle.sin() * 0.2 * fps as f32;
        }
    }
    if keys.down {
        // move the player backward.
        if map.wall_tiles[(ipy as i32 * map.width + ipx_sub_xo) as usize] == 0 {
            player.x -= player.angle.cos() * 0.2 * fps as f32;
        }

        if map.wall_tiles[(ipy_sub_yo * map.width + ipx as i32) as usize] == 0 {
            player.y -= player.angle.sin() * 0.2 * fps as f32;
        }
    }
    if keys.left {
        // turn the player to the left.
        player.angle -= ((0.2 * fps as f32) * PI) / 180.0;
        if player.angle < 0.0 {
            player.angle += 2.0 * PI;
        }
    }
    if keys.right {
        // turn the player to the right.
        player.angle += ((0.2 * fps as f32) * PI) / 180.0;
        if player.angle > 2.0 * PI {
            player.angle -= 2.0 * PI;
        }
    }
}

fn draw_sky(player: &Player, frame: &mut [u8]) {
    for y in 0..40 {
        for x in 0..120 {
            let mut x_offset = ((player.angle * (180.0 / PI)) * 2.0) as i32 + x;
            if x_offset < 0 {
                x_offset += 120;
            }

            x_offset %= 120;
            let pixel = ((y * 120 + x_offset) * 3) as usize;
            let red = SKY_DATA[pixel];
            let green = SKY_DATA[pixel + 1];
            let blue = SKY_DATA[pixel + 2];

            draw_pixel(red, green, blue, x as i32, y as i32, frame);
        }
    }
}

fn screen(screen_number: i32, fade: f32, frame: &mut [u8]) {
    for y in 0..80 {
        for x in 0..120 {
            let pixel = (((y * 120) + x) * 3) as usize;
            if screen_number == 1 {
                let red = (TITLE[pixel] as f32 * fade) as u8;
                let green = (TITLE[pixel + 1] as f32 * fade) as u8;
                let blue = (TITLE[pixel + 2] as f32 * fade) as u8;
                draw_pixel(red, green, blue, x as i32, y as i32, frame);
            }
            if screen_number == 2 {
                let red = (WON[pixel] as f32 * fade) as u8;
                let green = (WON[pixel + 1] as f32 * fade) as u8;
                let blue = (WON[pixel + 2] as f32 * fade) as u8;
                draw_pixel(red, green, blue, x as i32, y as i32, frame);
            }
            if screen_number == 3 {
                let red = (LOST[pixel] as f32 * fade) as u8;
                let green = (LOST[pixel + 1] as f32 * fade) as u8;
                let blue = (LOST[pixel + 2] as f32 * fade) as u8;
                draw_pixel(red, green, blue, x as i32, y as i32, frame);
            }
        }
    }
}

/// Cast the rays and draws the 3D view.
///
/// Raycasting algorithm is based on [Tutorial by 3DSage](https://youtu.be/gYRrGTC7GtA?list=PLMTDxt7L_MNXx7QP80seZUfcSoJ4jl34D&t=404).
///
fn draw_rays(player: &Player, map: &Map, frame: &mut [u8], depth: &mut [i32; 120]) {
    let mut mx;
    let mut my;
    let mut mp;
    let mut dof;
    let mut ray_x: f32 = 0.0;
    let mut ray_y: f32 = 0.0;
    let mut ray_angle: f32 = player.angle - DR * 30.0;
    if ray_angle < 0.0 {
        ray_angle += 2.0 * PI;
    }
    if ray_angle > 2.0 * PI {
        ray_angle -= 2.0 * PI;
    }
    let mut x_offset: f32 = 0.0;
    let mut y_offset: f32 = 0.0;
    let mut distance: f32 = 1.0;
    for r in 0..120 {
        let mut vmt = 0;
        let mut hmt = 0;

        dof = 0;
        let mut distance_h = 1000000.0;
        let mut horizontal_x = player.x;
        let mut horizontal_y = player.y;
        let a_tan = -1.0 / (ray_angle.tan());
        if ray_angle > PI {
            ray_y = ((player.y as i32 >> 6) << 6) as f32 - 0.0001;
            ray_x = (player.y - ray_y) * a_tan + player.x;
            y_offset = -64.0;
            x_offset = -y_offset * a_tan;
        }

        if ray_angle < PI {
            ray_y = ((player.y as i32 >> 6) << 6) as f32 + 64.0;
            ray_x = (player.y - ray_y) * a_tan + player.x;
            y_offset = 64.0;
            x_offset = -y_offset * a_tan;
        }

        if ray_angle == 0.0 || ray_angle == PI {
            ray_x = player.x;
            ray_y = player.y;
            dof = 8;
        }

        while dof < 8 {
            mx = (ray_x as i32) >> 6;
            my = (ray_y as i32) >> 6;

            mp = my * map.width + mx;
            if mp > 0 && mp < (map.width * map.height) && map.wall_tiles[mp as usize] > 0 {
                hmt = map.wall_tiles[mp as usize] - 1;
                horizontal_x = ray_x;
                horizontal_y = ray_y;
                distance_h = dist(player.x, player.y, horizontal_x, horizontal_y, ray_angle);
                dof = 8;
            } else {
                ray_x += x_offset;
                ray_y += y_offset;

                dof += 1;
            }
        }

        dof = 0;
        let mut distance_v = 1000000.0;
        let mut vertical_x = player.x;
        let mut vertical_y = player.y;
        let negative_tan = -ray_angle.tan();
        if ray_angle > P2 && ray_angle < P3 {
            ray_x = ((player.x as i32 >> 6) << 6) as f32 - 0.0001;
            ray_y = (player.x - ray_x) * negative_tan + player.y;
            x_offset = -64.0;
            y_offset = -x_offset * negative_tan;
        }

        if !(P2..=P3).contains(&ray_angle) {
            ray_x = ((player.x as i32 >> 6) << 6) as f32 + 64.0;
            ray_y = (player.x - ray_x) * negative_tan + player.y;
            x_offset = 64.0;
            y_offset = -x_offset * negative_tan;
        }

        if ray_angle == 0.0 || ray_angle == PI {
            ray_x = player.x;
            ray_y = player.y;
            dof = 8;
        }

        while dof < 8 {
            mx = (ray_x as i32) >> 6;
            my = (ray_y as i32) >> 6;

            mp = my * map.width + mx;
            if mp > 0 && mp < (map.width * map.height) && map.wall_tiles[mp as usize] > 0 {
                vmt = map.wall_tiles[mp as usize] - 1;
                vertical_x = ray_x;
                vertical_y = ray_y;
                distance_v = dist(player.x, player.y, vertical_x, vertical_y, ray_angle);
                dof = 8;
            } else {
                ray_x += x_offset;
                ray_y += y_offset;

                dof += 1;
            }
        }

        let mut shade: f32 = 1.0;

        if distance_v < distance_h {
            hmt = vmt;
            shade = 0.5;
            ray_x = vertical_x;
            ray_y = vertical_y;
            distance = distance_v;
        }

        if distance_v > distance_h {
            ray_x = horizontal_x;
            ray_y = horizontal_y;
            distance = distance_h;
        }

        let mut fixed_angle = player.angle - ray_angle;
        if fixed_angle < 0.0 {
            fixed_angle += 2.0 * PI;
        }

        if fixed_angle > 2.0 * PI {
            fixed_angle -= 2.0 * PI;
        }

        distance *= fixed_angle.cos();

        let mut line_h = ((TILE_SIZE * 80) as f32 / distance) as i32;

        let texture_y_step = 32.0 / line_h as f32;
        let mut texture_y_offset = 0.0;

        if line_h > 80 {
            texture_y_offset = (line_h - 80) as f32 / 2.0;
            line_h = 80;
        }

        let line_offset = 40 - (line_h >> 1);

        depth[r as usize] = distance as i32;

        // Drawing walls
        let mut texture_y: f32 = texture_y_offset * texture_y_step;
        let mut texture_x: f32;

        if shade == 1.0 {
            texture_x = (ray_x / 2.0) % 32.0;
            if ray_angle < PI {
                texture_x = 31.0 - texture_x;
            }
        } else {
            texture_x = (ray_y / 2.0) % 32.0;
            if ray_angle > PI / 2.0 && ray_angle < (270.0 * PI) / 180.0 {
                texture_x = 31.0 - texture_x;
            }
        }

        for y in 0..line_h {
            let pixel = ((texture_y as usize) * 32 + (texture_x) as usize) * 3
                + (hmt as usize * 32 * 32 * 3);
            let red = (RGB_TEXTURES[pixel] as f32 * shade) as u8;
            let green = (RGB_TEXTURES[pixel + 1] as f32 * shade) as u8;
            let blue = (RGB_TEXTURES[pixel + 2] as f32 * shade) as u8;

            draw_pixel(red, green, blue, r, y + line_offset, frame);

            texture_y += texture_y_step;
        }

        // Drawing floor
        for y in (line_offset + line_h)..80 {
            let delta_y = y as f32 - (80.0 / 2.0);
            let degree = ray_angle;

            let mut ray_angle_fix = player.angle - ray_angle;

            if ray_angle_fix < 0.0 {
                ray_angle_fix += 2.0 * PI;
            }

            if ray_angle_fix > (2.0 * PI) {
                ray_angle_fix -= 2.0 * PI;
            }

            ray_angle_fix = ray_angle_fix.cos();

            texture_x =
                player.x / 2.0 + degree.cos() * 158.0 * 0.25 * 32.0 / delta_y / ray_angle_fix;
            texture_y =
                player.y / 2.0 + degree.sin() * 158.0 * 0.25 * 32.0 / delta_y / ray_angle_fix;
            let mp = map.floor_tiles
                [((texture_y / 32.0) as i32 * map.width) as usize + (texture_x / 32.0) as usize]
                * 32
                * 32;

            let pixel = (((((texture_y as usize) & 31) * 32) + ((texture_x as usize) & 31))
                + mp as usize)
                * 3;
            let red = (RGB_TEXTURES[pixel] as f32 * 0.7) as u8;
            let green = (RGB_TEXTURES[pixel + 1] as f32 * 0.7) as u8;
            let blue = (RGB_TEXTURES[pixel + 2] as f32 * 0.7) as u8;
            draw_pixel(red, green, blue, r, y, frame);

            // Drawing ceiling
            let mp = map.ceiling_tiles
                [((texture_y / 32.0) as i32 * map.width) as usize + (texture_x / 32.0) as usize]
                * 32
                * 32;

            let pixel = (((((texture_y as usize) & 31) * 32) + ((texture_x as usize) & 31))
                + mp as usize)
                * 3;
            let red = RGB_TEXTURES[pixel];
            let green = RGB_TEXTURES[pixel + 1];
            let blue = RGB_TEXTURES[pixel + 2];
            if mp > 0 {
                draw_pixel(red, green, blue, r, 80 - y, frame);
            }
        }

        ray_angle += DR * 0.5;
        if ray_angle < 0.0 {
            ray_angle += 2.0 * PI;
        }

        if ray_angle > 2.0 * PI {
            ray_angle -= 2.0 * PI;
        }
    }
}

fn draw_sprite(
    sprite: &mut Sprite,
    player: &Player,
    depth: [i32; 120],
    game_state: &mut i32,
    fps: u128,
    map: &Map,
    frame: &mut [u8],
) {
    if player.x < (sprite.x + 30.0)
        && player.x > (sprite.x - 30.0)
        && player.y < (sprite.y + 30.0)
        && player.y > (sprite.y - 30.0)
        && sprite.npc_type == 1
    {
        sprite.state = 0;
    } else if player.x < (sprite.x + 30.0)
        && player.x > (sprite.x - 30.0)
        && player.y < (sprite.y + 30.0)
        && player.y > (sprite.y - 30.0)
        && sprite.npc_type == 3
    {
        *game_state = 4;
    }

    if sprite.npc_type == 3 {
        let spx = sprite.x as i32 >> 6;
        let spy = sprite.y as i32 >> 6;
        let spx_add = (sprite.x as i32 + 15) >> 6;
        let spy_add = (sprite.y as i32 + 15) >> 6;
        let spx_sub = (sprite.x as i32 - 15) >> 6;
        let spy_sub = (sprite.y as i32 - 15) >> 6;

        if sprite.x > player.x && map.wall_tiles[(spy * 8 + spx_sub) as usize] == 0 {
            sprite.x -= 0.03 * fps as f32;
        }

        if sprite.x < player.x && map.wall_tiles[(spy * 8 + spx_add) as usize] == 0 {
            sprite.x += 0.03 * fps as f32;
        }

        if sprite.y > player.y && map.wall_tiles[(spy_sub * 8 + spx) as usize] == 0 {
            sprite.y -= 0.03 * fps as f32;
        }

        if sprite.y < player.y && map.wall_tiles[(spy_add * 8 + spx) as usize] == 0 {
            sprite.y += 0.03 * fps as f32;
        }
    }

    let mut sx = sprite.x - player.x;
    let mut sy = sprite.y - player.y;
    let sz = sprite.z;

    let cs = player.angle.cos();
    let sn = player.angle.sin();

    let a = sy * cs - sx * sn;
    let b = sx * cs + sy * sn;
    sx = a;
    sy = b;

    sx = (sx * 108.0 / sy) + (120.0 / 2.0);
    sy = (sz * 108.0 / sy) + (80.0 / 2.0);

    let mut scale = 32.0 * 80.0 / b;
    if scale < 0.0 {
        scale = 0.0;
    }

    if scale > 120.0 {
        scale = 120.0;
    }

    let mut texture_x = 0.0;
    let texture_x_step = 31.5 / scale;
    let texture_y_step = 32.0 / scale;

    for x in (sx - (scale / 2.0)) as i32..(sx + (scale / 2.0)) as i32 {
        let mut texture_y = 31.0;
        for y in 0..scale as i32 {
            if x > 0 && x < 120 && (depth[x as usize] > b as i32 && sprite.state == 1) {
                let pixel = ((texture_y as usize) * 32 + (texture_x) as usize) * 3
                    + sprite.map as usize * 32 * 32 * 3;
                let red = SPRITES[pixel];
                let green = SPRITES[pixel + 1];
                let blue = SPRITES[pixel + 2];

                let draw_x = x;
                let draw_y = (sy as i32) - (y);

                if draw_y > 0 && draw_y < 80 && !(red == 255 && green == 0 && blue == 255) {
                    draw_pixel(red, green, blue, draw_x, draw_y, frame);
                }
                texture_y -= texture_y_step;
                if texture_y < 0.0 {
                    texture_y = 0.0;
                }
            }
        }
        texture_x += texture_x_step;
    }
}
