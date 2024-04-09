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

// This function is ported from C++ to Rust, it comes from my first ASCII graphics engine https://github.com/just-a-mango/mango-engine
/// Given two (x,y) points, draws a line from the first to the second point, using Bresenham's line algorithm
///
/// Arguments:
///
/// * `x1`: The x coordinate of the first point
/// * `y1`: The y coordinate of the first point
/// * `x2`: The x coordinate of the second point
/// * `y2`: The y coordinate of the second point
/// * `color`: The color of the line
///
/// Returns:
///
/// * `Vec<Pixel>`: A vector of pixels that represent the line.
pub fn draw_line(mut x1: i32, mut y1: i32, x2: i32, y2: i32, color: Color) -> Vec<Pixel> {
    let mut pixels: Vec<Pixel> = Vec::new();
    let dx: i32 = (x2 - x1).abs();
    let dy: i32 = (y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy: i32 = if y1 < y2 { 1 } else { -1 };
    let mut err: i32 = dx - dy;
    loop {
        pixels.push(Pixel {
            x: x1 as u16,
            y: y1 as u16,
            color,
        });
        if x1 == x2 && y1 == y2 { break };
        let e2: i32 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x1 += sx;
        }
        if e2 < dx {
            err += dx;
            y1 += sy;
        }
    }
    pixels
}


/// Given three (x,y) points, draws a triangle by drawing a line between each point.
///
/// Arguments:
///
/// * `x1`: The x coordinate of the first triangle point
/// * `y1`: The y coordinate of the first triangle point
/// * `x2`: The x coordinate of the second triangle point
/// * `y2`: The y coordinate of the second triangle point
/// * `x3`: The x coordinate of the third triangle point
/// * `y3`: The y coordinate of the third triangle point
/// * `color`: The color of the triangle
///
/// Returns:
///
/// * `Vec<Pixel>`: A vector of pixels that represent the line.
pub fn triangle(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: Color) -> Vec<Pixel> {
    let mut pixels: Vec<Pixel> = Vec::new();
    pixels.extend(draw_line(x1, y1, x2, y2, color));
    pixels.append(&mut draw_line(x1, y1, x3, y3, color));
    pixels.append(&mut draw_line(x2, y2, x3, y3, color));
    pixels
}