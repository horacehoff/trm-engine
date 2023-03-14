use crate::{Pixel, all_pixels_off, text::draw_text, rendering::{render, draw_line}};


pub fn begin_play(pixel_buffer: &Vec<Pixel>) {
    // insert here the logic for the game



    let mut out_buffer: Vec<Pixel> = pixel_buffer.clone();
    out_buffer = all_pixels_off(&out_buffer);
    let mut i = 1;
    let mut x = 0;
    draw_line(&mut out_buffer, 0, 0, 149, 0, "white");
    loop {
        out_buffer = all_pixels_off(&out_buffer);
        draw_line(&mut out_buffer, 34, 0, 34, 50, "white");
        draw_text(&mut out_buffer, 36, x, "TRM-ENGINE", "white");
        draw_line(&mut out_buffer, 101, 0, 101, 50, "white");

        i += 1;
        if i > 100 {
            i = 1;
            x += 1;
        }
        if x > 46 {
            x = 0;
        }




        // DO NOT REMOVE THIS LINE - KEEP IT AT THE END OF THE LOOP
        render(&out_buffer);
    }
}