use std::io::stdout;

use crossterm::{cursor::Hide, queue, style::Color};

use object::Object;
use rendering::Pixel;
use text::big_text;
use util::new_frame;

use crate::shapes::{circle, triangle};
use crate::util::random_color;

mod input;
mod object;
mod rendering;
mod shapes;
mod text;
mod util;

/// Demo function to show what can be done with this engine
/// Generates a vector of pixels that represent a 2D terrain
///
/// Arguments:
///
/// * `base_height`: the height of the terrain on the left side of the screen
/// * `width`: the width of the terrain
///
/// Returns:
///
/// A vector of pixels.
fn generate_terrain(base_height: i32, width: u16) -> Vec<Pixel> {
    let mut pts: Vec<Pixel> = Vec::new();
    let mut previous_height: i32 = 0;
    let mut height: i32 = 0;

    let mut rng = fastrand::Rng::new();

    for i in 0..width + 1 {
        if i == 0 {
            height = base_height;
            previous_height = height;
        }

        if height < 25 {
            height = previous_height + 1
        } else if height == 49 {
            height = previous_height - 1
        } else {
            // makes the terrain 50% more likely to be "plateau-y"
            // if rand::thread_rng().gen_range(0..2) == 1 {
            //     height = previous_height
            // } else {
            height = previous_height - (rng.i32(-1..2));

            // }
        }
        previous_height = height;

        pts.push(Pixel {
            x: i,
            y: height as u16,
            color: Color::Blue,
        })
    }
    pts
}

fn init() {
    queue!(stdout(), crossterm::terminal::SetSize(150, 300), Hide).unwrap();
    // enable raw mode, only do this if you plan on having the user interact with the terminal screen
    // crossterm::terminal::enable_raw_mode().unwrap();
    new_frame();
}

fn main() {
    init();

    let mut obj = Object::new(big_text(0, 35, "trm-engine", Color::Red));
    let terrain = Object::new(generate_terrain(10, 150));
    game_loop!({
        obj.draw();
        terrain.draw();
        // Below are examples of what you can do
        obj.move_object(0, if obj.pixels[0].y < 10 { 30 } else { -1 });
        obj.draw();
    });
}
