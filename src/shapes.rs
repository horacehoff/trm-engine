use crossterm::style::Color;
use crate::rendering::Pixel;

/// Given a starting point, a side length, and a color, return a vector of pixels that represent a
/// square.
/// 
/// Arguments:
/// 
/// * `x`: The x coordinate of the top left corner of the square
/// * `y`: The y coordinate of the top left corner of the square
/// * `side_length`: The length of the side of the square.
/// * `color`: The color of the square
/// 
/// Returns:
/// 
/// * `Vec<Pixel>`: A vector of pixels that represent the square.
pub fn square(x: i32, y: i32, side_length: i32, color: Color) -> Vec<Pixel> {
    let mut pixels: Vec<Pixel> = Vec::new();
    for i in 0..side_length*2 {
        for j in 0..side_length {
            pixels.push(Pixel { x: (x+i) as u16, y: (y+j) as u16, color })
        }
    }
    pixels
}

/// Given an x and y coordinate, a width and height, and a color, return a vector of pixels that
/// represent a rectangle.
/// 
/// Arguments:
/// 
/// * `x`: The x coordinate of the top left corner of the rectangle
/// * `y`: The y coordinate of the top left corner of the rectangle
/// * `width`: The width of the rectangle
/// * `height`: The height of the rectangle
/// * `color`: The color of the rectangle
/// 
/// Returns:
/// 
/// * `Vec<Pixel>`: A vector of pixels that represent the rectangle.
pub fn rectangle(x: i32, y: i32, width: i32, height: i32, color: Color) -> Vec<Pixel> {
    let mut pixels: Vec<Pixel> = Vec::new();
    for i in 0..width {
        for j in 0..height {
            pixels.push(Pixel { x: (x+i) as u16, y: (y+j) as u16, color })
        }
    }
    pixels
}