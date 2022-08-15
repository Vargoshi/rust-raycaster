extern crate sdl2;

pub mod texture_data;
use crate::texture_data::ALL_TEXTURES;

use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::pixels;

use sdl2::rect::{Point, Rect};

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 512;

const PI: f32 = 3.1415926535;
const P2: f32 = PI / 2.0;
const P3: f32 = 3.0 * PI / 2.0;
const DR: f32 = 0.0174533;
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

struct Keyboard {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

fn dist(ax: f32, ay: f32, bx: f32, by: f32, _ang: f32) -> f32 {
    ((bx - ax) * (bx - ax) + (by - ay) * (by - ay)).sqrt()
}

fn main() -> Result<(), String> {
    let mut map1 = Map {
        width: 8,
        height: 8,
        wall_tiles: vec![
            1, 1, 1, 1, 1, 3, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 4, 0, 2, 0, 1, 1, 1, 4, 1, 0,
            0, 0, 1, 2, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 0, 0, 1, 0, 1, 2, 0, 0, 0, 0, 0, 0, 1, 1, 1,
            3, 1, 3, 1, 3, 1,
        ],
        floor_tiles: vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ],
        ceiling_tiles: vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 1, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ],
    };

    let mut player1 = Player {
        x: 300.0,
        y: 300.0,
        angle: 0.0,
    };

    let mut keys = Keyboard {
        up: false,
        down: false,
        left: false,
        right: false,
    };

    let time = Instant::now();
    let mut frame1 = 0;
    let mut frame2;
    let mut fps;

    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window("Raycaster", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut events = sdl_context.event_pump()?;

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        break 'main;
                    }
                    if keycode == Keycode::Left {
                        keys.left = true;
                    }
                    if keycode == Keycode::Right {
                        keys.right = true;
                    }
                    if keycode == Keycode::Up {
                        keys.up = true;
                    }
                    if keycode == Keycode::Down {
                        keys.down = true;
                    }
                    if keycode == Keycode::E {
                        door_open(&player1, &mut map1);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Left {
                        keys.left = false;
                    }
                    if keycode == Keycode::Right {
                        keys.right = false;
                    }
                    if keycode == Keycode::Up {
                        keys.up = false;
                    }
                    if keycode == Keycode::Down {
                        keys.down = false;
                    }
                }
                _ => {}
            }
        }

        frame2 = time.elapsed().as_millis();
        fps = frame2 - frame1;
        frame1 = time.elapsed().as_millis();

        keyboard_input(&keys, &mut player1, fps, &map1);
        draw_map(&mut canvas, &map1)?;
        draw_player(&mut canvas, &player1)?;
        draw_rays(&player1, &map1, ALL_TEXTURES, &mut canvas)?;

        canvas.present();
    }

    Ok(())
}

fn door_open(player1: &Player, map1: &mut Map) {
    let x_offset;
    if player1.angle.cos() < 0.0 {
        x_offset = -25;
    } else {
        x_offset = 25;
    }
    let y_offset;
    if player1.angle.sin() < 0.0 {
        y_offset = -25;
    } else {
        y_offset = 25;
    }
    let ipx_add_xo = (player1.x as i32 + x_offset) / 64;
    let ipy_add_yo = (player1.y as i32 + y_offset) / 64;
    if map1.wall_tiles[(ipy_add_yo * map1.width + ipx_add_xo) as usize] == 4 {
        map1.wall_tiles[(ipy_add_yo * map1.width + ipx_add_xo) as usize] = 0;
    }
}

fn keyboard_input(keys: &Keyboard, player: &mut Player, fps: u128, map: &Map) {
    let x_offset;
    if player.angle.cos() < 0.0 {
        x_offset = -20;
    } else {
        x_offset = 20;
    }

    let y_offset;
    if player.angle.sin() < 0.0 {
        y_offset = -20;
    } else {
        y_offset = 20;
    }

    let ipx = player.x / 64.0;
    let ipx_add_xo = (player.x as i32 + x_offset) / 64;
    let ipx_sub_xo = (player.x as i32 - x_offset) / 64;
    let ipy = player.y / 64.0;
    let ipy_add_yo = (player.y as i32 + y_offset) / 64;
    let ipy_sub_yo = (player.y as i32 - y_offset) / 64;

    if keys.up == true {
        // move the player forward.

        if map.wall_tiles[(ipy as i32 * map.width + ipx_add_xo) as usize] == 0 {
            player.x += player.angle.cos() * 0.2 * fps as f32;
        }

        if map.wall_tiles[(ipy_add_yo * map.width + ipx as i32) as usize] == 0 {
            player.y += player.angle.sin() * 0.2 * fps as f32;
        }
    }
    if keys.down == true {
        // move the player backward.
        if map.wall_tiles[(ipy as i32 * map.width + ipx_sub_xo) as usize] == 0 {
            player.x -= player.angle.cos() * 0.2 * fps as f32;
        }

        if map.wall_tiles[(ipy_sub_yo * map.width + ipx as i32) as usize] == 0 {
            player.y -= player.angle.sin() * 0.2 * fps as f32;
        }
    }
    if keys.left == true {
        // turn the player to the left.
        player.angle -= ((0.2 * fps as f32) * PI) / 180.0;
        if player.angle < 0.0 {
            player.angle += 2.0 * PI;
        }
    }
    if keys.right == true {
        // turn the player to the right.
        player.angle += ((0.2 * fps as f32) * PI) / 180.0;
        if player.angle > 2.0 * PI {
            player.angle -= 2.0 * PI;
        }
    }
}

/// Cast the rays and draws the 3D view.
///
/// Raycasting algorithm is based on [Tutorial by 3DSage](https://youtu.be/gYRrGTC7GtA?list=PLMTDxt7L_MNXx7QP80seZUfcSoJ4jl34D&t=404).
///
fn draw_rays(
    player: &Player,
    map: &Map,
    textures: [i32; 4096],
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) -> Result<(), String> {
    let mut mx;
    let mut my;
    let mut mp;
    let mut mv = 0;
    let mut mh = 0;
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
    for r in 0..60 {
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
                mh = map.wall_tiles[mp as usize];
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

        if ray_angle < P2 || ray_angle > P3 {
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
                mv = map.wall_tiles[mp as usize];
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
            canvas.set_draw_color(pixels::Color::RGB(229, 0, 0));
            if mv == 2 {
                canvas.set_draw_color(pixels::Color::RGB(0, 0, 229));
            }
        }

        if distance_v > distance_h {
            ray_x = horizontal_x;
            ray_y = horizontal_y;
            distance = distance_h;
            canvas.set_draw_color(pixels::Color::RGB(178, 0, 0));
            if mh == 2 {
                canvas.set_draw_color(pixels::Color::RGB(0, 0, 178));
            }
        }

        canvas.draw_line(
            Point::new(player.x as i32, player.y as i32),
            Point::new(ray_x as i32, ray_y as i32),
        )?;

        let mut fixed_angle = player.angle - ray_angle;
        if fixed_angle < 0.0 {
            fixed_angle += 2.0 * PI;
        }

        if fixed_angle > 2.0 * PI {
            fixed_angle -= 2.0 * PI;
        }

        distance = distance * fixed_angle.cos();

        let mut line_h = ((TILE_SIZE * 320) as f32 / distance) as i32;

        let texture_y_step = 32.0 / line_h as f32;
        let mut texture_y_offset = 0.0;

        if line_h > 320 {
            texture_y_offset = (line_h - 320) as f32 / 2.0;
            line_h = 320;
        }

        let line_offset = 160 - (line_h >> 1);

        // Drawing walls
        let mut texture_y: f32 = texture_y_offset * texture_y_step + hmt as f32 * 32.0;

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
            let color =
                (textures[(texture_y as i32 * 32 + texture_x as i32) as usize]) as f32 * shade;

            if hmt == 0 {
                canvas.set_draw_color(pixels::Color::RGB(
                    (255.0 * color) as i32 as u8,
                    (255.0 * color / 2.0) as i32 as u8,
                    (255.0 * color / 2.0) as i32 as u8,
                ));
            }
            if hmt == 1 {
                canvas.set_draw_color(pixels::Color::RGB(
                    (255.0 * color) as i32 as u8,
                    (255.0 * color) as i32 as u8,
                    (255.0 * color / 2.0) as i32 as u8,
                ));
            }
            if hmt == 2 {
                canvas.set_draw_color(pixels::Color::RGB(
                    (255.0 * color / 2.0) as i32 as u8,
                    (255.0 * color / 2.0) as i32 as u8,
                    (255.0 * color) as i32 as u8,
                ));
            }
            if hmt == 3 {
                canvas.set_draw_color(pixels::Color::RGB(
                    (255.0 * color / 2.0) as i32 as u8,
                    (255.0 * color) as i32 as u8,
                    (255.0 * color / 2.0) as i32 as u8,
                ));
            }

            canvas.fill_rect(Rect::new(r * 8 + 530 - 4, y + line_offset, 8, 1))?;
            texture_y += texture_y_step;
        }

        // Drawing floor
        for y in (line_offset + line_h)..320 {
            let delta_y = y as f32 - (320.0 / 2.0);
            let degree = ray_angle;

            let mut ray_angle_fix = player.angle - ray_angle;

            if ray_angle_fix < 0.0 {
                ray_angle_fix += 2.0 * PI;
            }

            if ray_angle_fix > (2.0 * PI) {
                ray_angle_fix -= 2.0 * PI;
            }

            ray_angle_fix = ray_angle_fix.cos();

            texture_x = player.x / 2.0 + degree.cos() * 158.0 * 32.0 / delta_y / ray_angle_fix;
            texture_y = player.y / 2.0 + degree.sin() * 158.0 * 32.0 / delta_y / ray_angle_fix;
            let mp = map.floor_tiles
                [((texture_y / 32.0) as i32 * map.width) as usize + (texture_x / 32.0) as usize]
                * 32
                * 32;

            let color = (textures
                [((((texture_y as usize) & 31) * 32) + ((texture_x as usize) & 31)) + mp as usize])
                as f32
                * 0.7;
            canvas.set_draw_color(pixels::Color::RGB(
                (255.0 * color / 1.3) as i32 as u8,
                (255.0 * color / 1.3) as i32 as u8,
                (255.0 * color) as i32 as u8,
            ));
            canvas.fill_rect(Rect::new(r * 8 + 530 - 4, y, 8, 1))?;

            // Drawing ceiling
            let mp = map.ceiling_tiles
                [((texture_y / 32.0) as i32 * map.width) as usize + (texture_x / 32.0) as usize]
                * 32
                * 32;

            let color = (textures
                [((((texture_y as usize) & 31) * 32) + ((texture_x as usize) & 31)) + mp as usize])
                as f32
                * 0.7;
            canvas.set_draw_color(pixels::Color::RGB(
                (255.0 * color / 2.0) as i32 as u8,
                (255.0 * color / 1.2) as i32 as u8,
                (255.0 * color / 2.0) as i32 as u8,
            ));
            canvas.fill_rect(Rect::new(r * 8 + 530 - 4, 320 - y, 8, 1))?;
        }

        ray_angle += DR;
        if ray_angle < 0.0 {
            ray_angle += 2.0 * PI;
        }

        if ray_angle > 2.0 * PI {
            ray_angle -= 2.0 * PI;
        }
    }
    Ok(())
}

/// Renders the player in 2D.
fn draw_player(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    player: &Player,
) -> Result<(), String> {
    let player_dest_x = player.angle.cos() * 5.0;
    let player_dest_y = player.angle.sin() * 5.0;

    canvas.set_draw_color(pixels::Color::RGB(255, 255, 0));

    // draw the player body.
    canvas.fill_rect(Rect::new(
        (player.x - 4.0) as i32,
        (player.y - 4.0) as i32,
        8,
        8,
    ))?;

    //draw the player angle arrow.
    canvas.draw_line(
        Point::new(player.x as i32, player.y as i32),
        Point::new(
            (player.x + (player_dest_x * 5.0)) as i32,
            (player.y + (player_dest_y * 5.0)) as i32,
        ),
    )?;
    Ok(())
}

/// Draws all map tiles in 2D view.
fn draw_map(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    map: &Map,
) -> Result<(), String> {
    canvas.set_draw_color(pixels::Color::RGB(76, 76, 76));
    canvas.clear();
    for y in 0..map.height {
        for x in 0..map.width {
            if map.wall_tiles[(y * map.width + x) as usize] > 0 {
                canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
            } else {
                canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
            }
            let map_x_offset = x * TILE_SIZE as i32;
            let map_y_offset = y * TILE_SIZE as i32;
            canvas.fill_rect(Rect::new(
                map_x_offset as i32 + 1,
                map_y_offset as i32 + 1,
                (TILE_SIZE as u32) - 1,
                (TILE_SIZE as u32) - 1,
            ))?;
        }
    }
    Ok(())
}
