use opengl_graphics::{GlGraphics, OpenGL};
use graphics::{Context, Graphics};
use std::collections::HashMap;
use piston::window::{Window, WindowSettings};
use piston::input::*;
use piston::event_loop::*;

#[cfg(feature = "include_sdl2")]
use sdl2_window::Sdl2Window as AppWindow;

#[cfg(feature = "include_glfw")]
use glfw_window::GlfwWindow as AppWindow;

#[cfg(feature = "include_glutin")]
use glutin_window::GlutinWindow as AppWindow;

#[derive(Debug)]
struct Block {
    kind: u32,
    x: i32,
    y: i32,
    z: i32,
    flags: u32,
}

impl Block {
    fn from_hashmap(map: HashMap<prc::hash40::Hash40, prc::param::ParamKind>) -> Self {
        Self {
            kind: *map[&prc::hash40::to_hash40("kind")].unwrap(),
            x: *map[&prc::hash40::to_hash40("x")].unwrap(),
            y: *map[&prc::hash40::to_hash40("y")].unwrap(),
            z: *map[&prc::hash40::to_hash40("z")].unwrap(),
            flags: *map[&prc::hash40::to_hash40("flags")].unwrap(),
        }
    }
}

const BLOCK_SIZE: f64 = 20.0;

fn main() {
    let breakable_stdat = std::env::args().skip(1).next().unwrap();
    let param = prc::open(breakable_stdat).unwrap();

    let blocks: Vec<Block> = if let prc::param::ParamKind::List(list) = param.into_iter().next().unwrap().1 {
        list.into_iter()
            .map(|x| Block::from_hashmap(x.unwrap_as_hashmap().unwrap()))
            .collect()
    } else {
        panic!("Invalid structure")
    };

    let mut window: AppWindow = WindowSettings::new("Minecraft Viewing", [600, 600])
        .exit_on_esc(true).build().unwrap();

    let opengl = OpenGL::V3_2;
    let ref mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                    graphics::clear([1.0; 4], g);

                    let size = window.draw_size();
                    let height = size.height as isize;
                    let width = size.width as isize;
                    let width_center = (width / 2) as f64;
                    let vert_center = (height / 2) as f64;

                    for block in &blocks {
                        block.draw((width_center, vert_center), &c, g);
                    }
                    //draw_state(&state, &window, &c, g);
                }
            );
        }
    }
}

const DIRT_BROWN: [f32; 4] = [
    (0x85 as f32) / (255 as f32),
    (0x4F as f32) / (255 as f32),
    (0x2B as f32) / (255 as f32),
    1.0,
];

impl Block {
    fn draw<G: Graphics>(&self, center: (f64, f64), c: &Context, g: &mut G) {
        let rect = graphics::Rectangle::new_round(
            DIRT_BROWN,
            2.0
        );

        rect.draw(
            [
                (center.0 + ((-self.x as f64) * BLOCK_SIZE) - (BLOCK_SIZE / 2.0)),
                (center.1 + ((-self.y as f64) * BLOCK_SIZE) - (BLOCK_SIZE / 2.0)),
                BLOCK_SIZE,
                BLOCK_SIZE
            ],
            &c.draw_state,
            c.transform,
            g
        );
    }
}
