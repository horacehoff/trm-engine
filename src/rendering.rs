use std::io::{stdout, Write};

use crossterm::{style::{Color, Attribute, SetAttribute, SetForegroundColor, ResetColor}, queue, cursor::MoveTo};

use crate::{util::GLOBAL_PIXEL_DATA, access_pixel_data};

/// A `Pixel` is a struct with three fields: `x`, `y`, and `color`.
/// 
/// The `x` and `y` fields are both unsigned 16-bit integers. The `color` field is a `Color` type.
/// 
/// The `Color` type is defined in the `crossterm` crate.
/// 
/// Properties:
/// 
/// * `x`: The x coordinate of the pixel.
/// * `y`: The y coordinate of the pixel.
/// * `color`: The color of the pixel.
#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub x: u16,
    pub y: u16,
    pub color: Color,
}



/// Takes a position, a color, a string and an optional vector of attributes and draws the string to
/// the terminal
/// 
/// Arguments:
/// 
/// * `x`: u16, y: u16 - The position of the text on the terminal
/// * `y`: The y coordinate of the pixel
/// * `color`: The color of the text
/// * `text`: The text to be drawn
/// * `attribute`: This is an optional vector of attributes that you can apply to the text.
pub fn draw_text_to_terminal(x: u16, y: u16, color:Color, text: &str, attribute: Option<Vec<Attribute>>) {
    if x > 150 || y > 50 {
        panic!("Text position exceeds terminal limit (150;50) - x: {}, y: {}", x, y)
    }
    for attr in attribute.iter() {
        for a in attr.iter() {
            queue!(stdout(), SetAttribute(*a)).unwrap();
        }
    }
    queue!(
        stdout(),
        MoveTo(x, y),
        SetForegroundColor(color),
        crossterm::style::Print(text),
        ResetColor,
        SetAttribute(Attribute::Reset)
    ).expect("Failed to draw to terminal");

    stdout().flush().unwrap()
}



/// Takes a position and a color, and draws a pixel to the terminal
/// 
/// Arguments:
/// 
/// * `x`: u16, y: u16 - The position of the pixel on the terminal
/// * `y`: u16, x: u16, color:Color
/// * `color`: The color of the pixel.
pub fn draw_to_terminal(x: u16, y: u16, color:Color) {
    if x > 150 || y > 50 {
        panic!("Pixel position exceeds terminal limit (150;50) - x: {}, y: {}", x, y)
    }
    GLOBAL_PIXEL_DATA.lock().unwrap().push(Pixel { x, y, color });
    queue!(
        stdout(),
        MoveTo(x, y),
        SetForegroundColor(color),
        crossterm::style::Print("█"),
        ResetColor
    ).expect("Failed to draw to terminal");

    stdout().flush().unwrap()
}

pub fn draw_vector_to_terminal(pixels: Vec<Pixel>) {
    for pixel in pixels {
        draw_to_terminal(pixel.x, pixel.y, pixel.color);
    }
}

pub fn erase_pixel(x: u16, y: u16) {
    if x > 150 || y > 50 {
        panic!("Pixel position exceeds terminal limit (150;50) - x: {}, y: {}", x, y)
    }
    GLOBAL_PIXEL_DATA.lock().unwrap().retain(|pxl| pxl.x != x && pxl.y != y);
    queue!(
        stdout(),
        MoveTo(x, y),
        crossterm::style::Print(" "),
    ).expect("Failed to erase pixel");

    stdout().flush().unwrap()
}

/// Loops through all the pixels in the pixel data, and if the pixel is within the specified area, it
/// draws a space character to the terminal, effectively erasing the pixel.
/// 
/// Arguments:
/// 
/// * `x1`: The x coordinate of the top left corner of the area to be erased.
/// * `y1`: The y coordinate of the top left corner of the area to be erased.
/// * `x2`: The x coordinate of the bottom right corner of the area to be erased.
/// * `y2`: The y coordinate of the bottom right corner of the area to be erased.
pub fn erase_pixel_area(x1: u16, y1: u16, x2: u16, y2: u16) {
    for pxl in access_pixel_data!() {
        if pxl.x >= x1 && pxl.x <= x2 && pxl.y >= y1 && pxl.y <= y2 {
            queue!(
                stdout(),
                MoveTo(pxl.x, pxl.y),
                crossterm::style::Print(" ")
            ).expect("Failed to draw to terminal");
        }
    }
}