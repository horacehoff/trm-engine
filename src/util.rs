use std::{io::stdout, sync::Mutex};

use crossterm::queue;
use crossterm::style::Color;

use crate::rendering::Pixel;

/// Global variable that stores the pixels on the screen at any time
/// * Not recommended to use this variable directly, use the `access_pixel_data!` macro instead
/// * This variable is only used for reading the pixel data, writing to it will not change the terminal and will only probably break the engine
pub static GLOBAL_PIXEL_DATA: Mutex<Vec<Pixel>> = Mutex::new(Vec::new());



/// Macro that allows easy access the global pixel data.
#[macro_export]
macro_rules! access_pixel_data {
    () => {
        &*GLOBAL_PIXEL_DATA.lock().unwrap()
    };
}


#[macro_use]
mod util {}


/// Completely clears the terminal and clears the global pixel data, causes a 'jump' in the terminal
pub fn new_frame() {
    queue!(stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
    GLOBAL_PIXEL_DATA.lock().unwrap().clear();
}

/// Clears the terminal by drawing a space over every pixel and clearing the global pixel data
pub fn erase_terminal() {
    for pixel in access_pixel_data!() {
        queue!(stdout(), crossterm::cursor::MoveTo(pixel.x, pixel.y), crossterm::style::Print(" ")).unwrap();
    }
    GLOBAL_PIXEL_DATA.lock().unwrap().clear();
}

/// Starts the main rendering/logic loop, at 20 frames per second
#[macro_export]
macro_rules! game_loop {
    ($a:expr) => {
        loop {
            $a;
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    };
}

/// Generates a random color
pub fn random_color() -> Color {
    let colors = vec![Color::Black, Color::Blue, Color::Cyan, Color::DarkBlue, Color::DarkCyan, Color::DarkGreen, Color::DarkGrey, Color::DarkMagenta, Color::DarkRed, Color::DarkYellow, Color::Green, Color::Grey, Color::Magenta, Color::Red, Color::White, Color::Yellow];
    let index = fastrand::usize(..colors.len());
    colors[index]
}

/// Generates a random RGB color
pub fn random_rgb_color() -> Color {
    let r = fastrand::i16(0..255);
    let g = fastrand::i16(0..255);
    let b = fastrand::i16(0..255);
    Color::Rgb {
        r: r as u8,
        g: g as u8,
        b: b as u8,
    }
}