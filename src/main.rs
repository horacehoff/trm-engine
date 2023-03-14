use std::io::{stdout, Write};
use crossterm::{cursor::MoveTo, queue, terminal::SetSize};

use crate::{rendering::render, logic::begin_play};
mod rendering;
mod bresenham;
mod text;
mod logic;

#[derive(PartialEq, Eq, Clone)]
pub struct Pixel {
    lit: bool,
    color: String,
    x: u8,
    y: u8
}

fn all_pixels_off(buffer: &Vec<Pixel>) -> Vec<Pixel> {
    let mut out_buffer: Vec<Pixel> = buffer.clone();
    for pixel in &mut out_buffer {
        pixel.lit = false;
    }
    out_buffer
}

fn all_pixels_on(buffer: &Vec<Pixel>, color: Option<&str>) -> Vec<Pixel> {
    let mut out_buffer: Vec<Pixel> = buffer.clone();
    for pixel in &mut out_buffer {
        pixel.lit = true;
        pixel.color = color.unwrap_or("white").to_string();
    }
    out_buffer
}

fn clear_terminal() {
    // clear the terminal and purge using crossterm
    crossterm::terminal::Clear(crossterm::terminal::ClearType::Purge);
}



fn logic() {
    let mut pixel_buffer: Vec<Pixel> = Vec::new();
    let width: u16 = 150;
    let height: u16 = 50;
    
    for y in 0..height {
        for x in 0..width {
            let pixel = Pixel {
                lit: true,
                color: "white".to_string(),
                x: x as u8,
                y: y as u8
            };
            pixel_buffer.push(pixel);
        }
    }
    // move cursor to top left
    queue!(stdout(), MoveTo(0, 0), SetSize(150, 50)).unwrap();
    render(&pixel_buffer);
    begin_play(&pixel_buffer);
}

fn main() {
    clear_terminal();
    logic();
    stdout().lock().write_fmt(format_args!("\x1B[3J")).unwrap();
}
