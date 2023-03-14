use std::{io::{stdout, Write}, sync::Mutex};
use crate::{Pixel, clear_terminal, bresenham};




#[inline(always)]
pub fn draw<'a>(buffer: &'a mut Vec<Pixel>, x: u16, y: u16, color: &str) -> &'a Vec<Pixel> {
    buffer[(y * 150 + x) as usize].lit = true;
    buffer[(y * 150 + x) as usize].color = color.to_string();
    buffer
}

#[inline(always)]
pub fn draw_line<'a>(buffer: &'a mut Vec<Pixel>, x1: u16, y1: u16, x2: u16, y2:u16, color: &str) -> &'a Vec<Pixel> {
    for (x, y) in bresenham::Bresenham::new((x1 as isize, y1 as isize), (x2 as isize, y2 as isize)) {
        draw(buffer, x as u16, y as u16, color);
    }
    buffer
}

macro_rules! color {
    ($color:expr) => {
        if $color == "white" {
            print!("{}[37m", 27 as char);
        } else if $color == "red" {
            print!("{}[31m", 27 as char);
        } else if $color == "green" {
            print!("{}[32m", 27 as char);
        } else if $color == "blue" {
            print!("{}[34m", 27 as char);
        } else if $color == "yellow" {
            print!("{}[33m", 27 as char);
        } else if $color == "magenta" {
            print!("{}[35m", 27 as char);
        } else if $color == "cyan" {
            print!("{}[36m", 27 as char);
        } else if $color == "black" {
            print!("{}[30m", 27 as char);
        } else if $color == "gray" {
            print!("{}[90m", 27 as char);
        } else if $color == "red" {
            print!("{}[91m", 27 as char);
        }
    };
}

static mut PREVIOUS_BUFFER: Option<Vec<Pixel>> = None;
static BUFFER_MUTEX: Mutex<()> = Mutex::new(());



fn update_previous_buffer(buffer: Vec<Pixel>) -> u8 {
    unsafe {
        let temp_buffer = buffer.clone();
        let _guard = BUFFER_MUTEX.lock().unwrap();
        if PREVIOUS_BUFFER.eq(&Some(buffer)) {
            1
        } else {
            PREVIOUS_BUFFER = Some(temp_buffer);
            0
        }
    }
}


pub fn render(buffer: &Vec<Pixel>) {
    stdout().lock().write_fmt(format_args!("\x1B[3J")).unwrap();


    let temp_buffer = buffer.clone();
    if update_previous_buffer(temp_buffer) == 1 {
        return;
    }

    // clear the terminal
    clear_terminal();
    // render the pixel buffer to the terminal
    // if the terminal is not the size of the canvas, print out a centered 150x50 pixel grid
    if crossterm::terminal::size().unwrap().0 != 150 || crossterm::terminal::size().unwrap().1 != 50 {
        let mut i = 0;
        for _ in 0..50 {
            for _ in 0..(crossterm::terminal::size().unwrap().0 - 150) / 2 {
                print!(" ");
            }
            for _ in 0..150 {
                if buffer[i].lit {
                    color!(buffer[i].color);
                    print!("█");
                } else {
                    print!(" ");
                }
                i += 1;
            }
            print!("\n");
        }
    } else {
        let mut x = 0;
        let mut i = 0;
        for pixel in buffer {
            if pixel.lit {
                color!(pixel.color);
                print!("█");
            } else {
                print!(" ");
            }
            
            x += 1;
            if x == 150 && i < 49 {
                x = 0;
                print!("\n");
                i += 1;
            }
        }
    }
    stdout().flush().unwrap();
}