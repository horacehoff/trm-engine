use std::io::stdout;
use crossterm::{queue, style::{Color, Print}, cursor::{Hide, Show}, event::{read, Event, KeyEvent, KeyModifiers, KeyCode}, execute, terminal::{ClearType, Clear, disable_raw_mode}};
use object::Object;
use rendering::{Pixel, draw_text_to_terminal, erase_pixel_area};
use shapes::{square, rectangle};
use text::{big_text, draw_big_text};
use util::new_frame;

mod rendering;
mod util;
mod text;
mod object;
mod shapes;



/// Generates a vector of pixels that represent a 2D terrain
/// 
/// Arguments:
/// 
/// * `base_height`: the height of the terrain at the left side of the screen
/// * `width`: the width of the terrain
/// 
/// Returns:
/// 
/// A vector of pixels.
fn generate_terrain(base_height: i32, width: u16) -> Vec<Pixel> {
    let mut pts: Vec<Pixel> = Vec::new();
    let mut previous_height: i32 = 0;
    let mut height: i32 = 0;

    let rng = fastrand::Rng::new();

    for i in 0..width+1 {
        if i == 0 {
            height = base_height;
            previous_height = height;
        }

        if height < 25 {
            height = previous_height+1
        } else if height == 49{
            height = previous_height-1
        } else {
            // makes the terrain 50% more likely to be "plateauy"
            // if rand::thread_rng().gen_range(0..2) == 1 {
            //     height = previous_height
            // } else {
                height = previous_height-(rng.i32(-1..2));


            // }
        }
        previous_height = height;

        pts.push(Pixel { x: i, y: height as u16, color: Color::DarkGrey })
    }
    pts
}



fn init() {
    queue!(stdout(), crossterm::terminal::SetSize(150, 300), Hide).unwrap();
    // enable raw mode
    // crossterm::terminal::enable_raw_mode().unwrap();
    new_frame();
}

fn main() {
    init();



    let mut obj = Object::new(big_text(0, 25, "trm-engine", Color::White));
    obj.draw();

    let mut i = 0;
    loop {
        i += 1;
        if (obj.pixels[0].x as i32) > 50 {
            obj.move_object(-50, 0)
        }
        // move object
        obj.move_object(1, 0);
        // wait one tenth of a second
        std::thread::sleep(std::time::Duration::from_millis(100));
        

    }
    // disable_raw_mode().unwrap();
}
