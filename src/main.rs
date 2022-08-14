extern crate sdl2;

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
    tiles: Vec<i32>,
}

struct Player {
    x: f32,
    y: f32,
    angle: f32,
}

fn dist(ax: f32, ay: f32, bx: f32, by: f32, _ang: f32) -> f32 {
    ((bx - ax) * (bx - ax) + (by - ay) * (by - ay)).sqrt()
}

fn main() -> Result<(), String> {
    let map1 = Map {
        width: 8,
        height: 8,
        tiles: vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0,
            0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1,
            1, 1, 1, 1, 1, 1,
        ],
    };

    let mut player1 = Player {
        x: 300.0,
        y: 300.0,
        angle: 0.0,
    };

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
                        // turn the player to the left.
                        player1.angle -= 0.1;
                        if player1.angle < 0.0 {
                            player1.angle += 2.0 * PI;
                        }
                    }
                    if keycode == Keycode::Right {
                        // turn the player to the right.
                        player1.angle += 0.1;
                        if player1.angle > 2.0 * PI {
                            player1.angle -= 2.0 * PI;
                        }
                    }
                    if keycode == Keycode::Up {
                        // move the player forward.
                        player1.x += player1.angle.cos() * 5.0;
                        player1.y += player1.angle.sin() * 5.0;
                    }
                    if keycode == Keycode::Down {
                        // move the player backward.
                        player1.x -= player1.angle.cos() * 5.0;
                        player1.y -= player1.angle.sin() * 5.0;
                    }
                }
                _ => {}
            }
        }

        draw_map(&mut canvas, &map1)?;
        draw_player(&mut canvas, &player1)?;
        draw_rays(&player1, &map1, &mut canvas)?;

        canvas.present();
    }

    Ok(())
}

/// Cast the rays and draws the 3D view.
/// 
/// Raycasting algorithm is based on [Tutorial by 3DSage](https://youtu.be/gYRrGTC7GtA?list=PLMTDxt7L_MNXx7QP80seZUfcSoJ4jl34D&t=404).
///
fn draw_rays(
    player: &Player,
    map: &Map,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) -> Result<(), String> {
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
    for r in 0..60 {
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
            if mp > 0 && mp < (map.width * map.height) && map.tiles[mp as usize] > 0 {
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
            if mp > 0 && mp < (map.width * map.height) && map.tiles[mp as usize] > 0 {
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

        if distance_v < distance_h {
            ray_x = vertical_x;
            ray_y = vertical_y;
            distance = distance_v;
            canvas.set_draw_color(pixels::Color::RGB(229, 0, 0));
        }
        if distance_v > distance_h {
            ray_x = horizontal_x;
            ray_y = horizontal_y;
            distance = distance_h;
            canvas.set_draw_color(pixels::Color::RGB(178, 0, 0));
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

        let mut line_h = (TILE_SIZE as f32 * 320.0) / distance;
        if line_h > 320.0 {
            line_h = 320.0;
        }

        let line_offset = 160.0 - line_h / 2.0;

        for line_width in -4..4 {
            canvas.draw_line(
                Point::new(r * 8 + 530 + line_width, line_offset as i32),
                Point::new(r * 8 + 530 + line_width, (line_h + line_offset) as i32),
            )?;
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
            if map.tiles[(y * map.width + x) as usize] == 1 {
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
