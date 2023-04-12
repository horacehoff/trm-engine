use crossterm::style::Color;

use crate::rendering::{draw_to_terminal, Pixel};


/// Draws big text to the terminal (letters are drawn using 'pixels')
/// 
/// DIFFERENT FROM BIG_TEXT FUNCTION:
/// - This function directly draws to the terminal
/// - The big_text function returns a vector of pixels that represent the text
/// 
/// Arguments:
/// 
/// * `x`: The x coordinate of the top left corner of the text
/// * `y`: The y coordinate of the top left corner of the text
/// * `text`: The text to draw
/// * `color`: The color of the text.
pub fn draw_big_text(x: u16, y: u16, text: &str, color: Color) {
    let mut x_offset = 0;
    for c in text.to_uppercase().chars() {
        if c == 'A' {
            for i in 0..3 {
                draw_to_terminal(x + x_offset, y + i + 1, color);
            }
            for i in 0..3 {
                draw_to_terminal(x + x_offset + 1 + i, y, color);
            }
            for i in 0..3 {
                draw_to_terminal(x + x_offset + 1 + i, y + 2, color);
            }
            for i in 0..3 {
                draw_to_terminal(x + x_offset + 4, y + i + 1, color);
            }
            x_offset += 7;
        } else if c == 'B' {
            for i in 0..4 {
                draw_to_terminal(x + x_offset, y + i, color);
            }
            for i in 0..3 {
                draw_to_terminal(x + x_offset + 1 + i, y, color);
            }
            draw_to_terminal(x + x_offset + 1, y + 1, color);
            draw_to_terminal(x + x_offset + 2, y + 1, color);
            draw_to_terminal(x + x_offset + 3, y + 2, color);
            draw_to_terminal(x + x_offset + 1, y + 3, color);
            draw_to_terminal(x + x_offset + 2, y + 3, color);
            x_offset += 6;
        } else if c == 'C' {
            for i in 0..2 {
                draw_to_terminal(x + x_offset, y + i + 1, color);
            }
            for i in 0..4 {
                draw_to_terminal(x + x_offset + 1 + i, y, color);
            }
            for i in 0..4 {
                draw_to_terminal(x + x_offset + 1 + i, y + 3, color);
            }
            
            x_offset += 7;
        } else if c == 'D' {
            for i in 0..4 {
                draw_to_terminal(x + x_offset, y + i, color);
            }
            for i in 0..3 {
                draw_to_terminal(x + x_offset + 1 + i, y, color);
            }
            for i in 0..3 {
                draw_to_terminal(x + x_offset + 1 + i, y + 3, color);
            }
            for i in 0..2 {
                draw_to_terminal(x + x_offset + 4, y + 1 + i, color);
            }

            x_offset += 7;
        } else if c == 'E' {
            for i in 0..4 {
                draw_to_terminal(x + x_offset, y + i, color);
            }
            for i in 0..4 {
                draw_to_terminal(x + x_offset + 1 + i, y, color);
            }
            for i in 0..2 {
                draw_to_terminal(x + x_offset + 1 + i, y + 1, color);
            }
            for i in 0..4 {
                draw_to_terminal(x + x_offset + 1 + i, y + 3, color);
            }
            x_offset += 7;
        } else if c == 'F' {
            for i in 0..4 {
                draw_to_terminal(x + x_offset, y + i, color);
            }
            for i in 0..4 {
                draw_to_terminal(x + x_offset + 1 + i, y, color);
            }
            for i in 0..2 {
                draw_to_terminal(x + x_offset + 1 + i, y + 1, color);
            }
            x_offset += 7;
        } else if c == 'G' {
            for i in 0..2 {
                draw_to_terminal(x + x_offset, y + i + 1, color);
            }
            for i in 0..4 {
                draw_to_terminal(x + x_offset + i + 1, y, color);
            }
            for i in 0..4 {
                draw_to_terminal(x + x_offset + i + 1, y + 3 , color);
            }
            draw_to_terminal(x + x_offset + 3, y + 2, color);
            draw_to_terminal(x + x_offset + 4, y + 2, color);
            x_offset += 7;
        } else if c == 'H' {
            for i in 0..4 {
                draw_to_terminal(x + x_offset, y + i, color);
            }
            for i in 0..4 {
                draw_to_terminal(x + x_offset + 4, y + i, color);
            }
            for i in 0..3 {
                draw_to_terminal(x + x_offset + 1 + i, y + 1, color);
            }
            x_offset += 7;
        } else if c == 'I' {
            for i in 0..4 {
                draw_to_terminal(x + x_offset + 1, y + i, color);
            }
            for i in 0..3 {
                draw_to_terminal(x + x_offset + i, y + 3, color);
            }
            for i in 0..3 {
                draw_to_terminal(x + x_offset + i, y, color);
            }
            x_offset += 5;
        } else if c == 'J' {
            for i in 0..4 {
                draw_to_terminal(x + x_offset + 1 + i, y, color);
            }
            for i in 0..3 {
                draw_to_terminal(x + x_offset + 3, y + i, color);
            }
            draw_to_terminal(x + x_offset + 2, y + 3, color);
            draw_to_terminal(x + x_offset + 1, y + 3, color);
            draw_to_terminal(x + x_offset, y + 2, color);
            x_offset += 7;
        } else if c == 'K' {
            for i in 0..4 {
                draw_to_terminal(x + x_offset, y + i, color);
            }
            for i in 0..2 {
                draw_to_terminal(x + x_offset + 1 + i, y + 1, color);
            }
            draw_to_terminal(x + x_offset + 3, y + 2, color);
            draw_to_terminal(x + x_offset + 4, y + 3, color);
            draw_to_terminal(x + x_offset + 3, y + 1, color);
            draw_to_terminal(x + x_offset + 4, y, color);
            x_offset += 7;
        } else if c == 'L' {
            for i in 0..4 {
                draw_to_terminal(x + x_offset, y + i, color);
            }
            for i in 0..4 {
                draw_to_terminal(x + x_offset + 1 + i, y + 3, color);
            }
            x_offset += 7;
        }
        else if c == 'M' {
            for i in 0..4 {
                draw_to_terminal(x + x_offset, y + i, color);
            }
            for i in 0..4 {
                draw_to_terminal(x + x_offset + 4, y + i, color);
            }
            draw_to_terminal(x + x_offset + 1, y + 1, color);
            draw_to_terminal(x + x_offset + 1, y + 1, color);
            draw_to_terminal(x + x_offset + 2, y + 2, color);
            draw_to_terminal(x + x_offset + 2, y + 2, color);
            draw_to_terminal(x + x_offset + 3, y + 1, color);
            draw_to_terminal(x + x_offset + 3, y + 1, color);
            x_offset += 7;
        } else if c == 'N' {
            for i in 0..4 {
                draw_to_terminal(x + x_offset, y + i, color);
            }
            for i in 0..4 {
                draw_to_terminal(x + x_offset + 4, y + i, color);
            }
            draw_to_terminal(x + x_offset + 1, y + 1, color);
            draw_to_terminal(x + x_offset + 1, y + 1, color);
            draw_to_terminal(x + x_offset + 2, y + 2, color);
            draw_to_terminal(x + x_offset + 2, y + 2, color);
            draw_to_terminal(x + x_offset + 3, y + 3, color);
            draw_to_terminal(x + x_offset + 3, y + 3, color);
            x_offset += 7;
        } else if c == 'O' {
            for i in 0..3 {
                draw_to_terminal(x + x_offset + 1 + i, y, color);
            }
            for i in 0..3 {
                draw_to_terminal(x + x_offset + 1 + i, y + 3, color);
            }
            for i in 0..2 {
                draw_to_terminal(x + x_offset, y + i + 1, color);
            }
            for i in 0..2 {
                draw_to_terminal(x + x_offset + 4, y + i + 1, color);
            }
            x_offset += 7;
        } else if c == 'P' {
            for i in 0..4 {
                draw_to_terminal(x + x_offset, y + i, color);
            }
            for i in 0..2 {
                draw_to_terminal(x + x_offset + 1 + i, y, color);
            }
            draw_to_terminal(x + x_offset + 3, y + 1, color);
            draw_to_terminal(x + x_offset + 3, y + 1, color);
            draw_to_terminal(x + x_offset + 1, y + 2, color);
            draw_to_terminal(x + x_offset + 1, y + 2, color);
            x_offset += 6;
        } else if c == 'Q' {
            for i in 0..4 {
                draw_to_terminal(x + x_offset + 1 + i, y, color);
            }
            for i in 0..4 {
                draw_to_terminal(x + x_offset + 1 + i, y + 3, color);
            }
            for i in 0..2 {
                draw_to_terminal(x + x_offset, y + i + 1, color);
            }
            for i in 0..2 {
                draw_to_terminal(x + x_offset + 5, y + i + 1, color);
            }
            draw_to_terminal(x + x_offset + 4, y + 2, color);
            draw_to_terminal(x + x_offset + 4, y + 2, color);
            x_offset += 8;
        }
        else if c == 'R' {
            for i in 0..4 {
                draw_to_terminal(x + x_offset, y + i, color);
            }
            for i in 0..2 {
                draw_to_terminal(x + x_offset + 1 + i, y, color);
            }
            draw_to_terminal(x + x_offset + 3, y + 1, color);
            draw_to_terminal(x + x_offset + 3, y + 1, color);
            draw_to_terminal(x + x_offset + 2, y + 2, color);
            draw_to_terminal(x + x_offset + 2, y + 2, color);
            draw_to_terminal(x + x_offset + 3, y + 3, color);
            draw_to_terminal(x + x_offset + 3, y + 3, color);
            x_offset += 6;
        } else if c == 'S' {
            for i in 0..4 {
                draw_to_terminal(x + x_offset + i, y, color);
            }
            for i in 0..2 {
                draw_to_terminal(x + x_offset + i, y + 1, color);
            }
            for i in 0..2 {
                draw_to_terminal(x + x_offset + i + 2, y + 2, color);
            }
            for i in 0..4 {
                draw_to_terminal(x + x_offset + i, y + 3, color);
            }
            x_offset += 6;
        }
        else if c == 'T' {
            for i in 0..5 {
                draw_to_terminal(x + x_offset + i, y, color);
            }
            for i in 0..4 {
                draw_to_terminal(x + x_offset + 2, y + i, color);
            }
            x_offset += 7;
        } else if c == 'U' {
            for i in 0..4 {
                draw_to_terminal(x + x_offset, y + i, color);
            }
            for i in 0..4 {
                draw_to_terminal(x + x_offset + 4, y + i, color);
            }
            for i in 0..3 {
                draw_to_terminal(x + x_offset + 1 + i, y + 3, color);
            }
            x_offset += 7;
        } else if c == 'V' {
            draw_to_terminal(x + x_offset, y, color);
            draw_to_terminal(x + x_offset, y + 1, color);
            draw_to_terminal(x + x_offset + 1, y + 2, color);
            draw_to_terminal(x + x_offset + 2, y + 3, color);
            draw_to_terminal(x + x_offset + 3, y + 2, color);
            draw_to_terminal(x + x_offset + 4, y + 1, color);
            draw_to_terminal(x + x_offset + 4, y, color);
            x_offset += 7;
        } else if c == 'W' {
            // first v
            draw_to_terminal(x + x_offset, y, color);
            draw_to_terminal(x + x_offset, y + 1, color);
            draw_to_terminal(x + x_offset + 1, y + 2, color);
            draw_to_terminal(x + x_offset + 2, y + 3, color);
            draw_to_terminal(x + x_offset + 3, y + 2, color);
            draw_to_terminal(x + x_offset + 4, y + 1, color);
            draw_to_terminal(x + x_offset + 4, y, color);

            // second v
            draw_to_terminal(x + x_offset + 4, y, color);
            draw_to_terminal(x + x_offset + 4, y + 1, color);
            draw_to_terminal(x + x_offset + 5, y + 2, color);
            draw_to_terminal(x + x_offset + 6, y + 3, color);
            draw_to_terminal(x + x_offset + 7, y + 2, color);
            draw_to_terminal(x + x_offset + 8, y + 1, color);
            draw_to_terminal(x + x_offset + 8, y, color);
            x_offset += 11
        } else if c == 'X' {
            for i in 0..4 {
                draw_to_terminal(x + x_offset + i, y + i, color);
            }
            for i in 0..4 {
                draw_to_terminal(x + x_offset + i, y + 3 - i, color);
            }
            x_offset += 6;
        } else if c == 'Y' {
            for i in 0..3 {
                draw_to_terminal(x + x_offset + i, y + i, color);
            }
            draw_to_terminal(x + x_offset + 2, y + 3, color);
            draw_to_terminal(x + x_offset + 3, y + 1, color);
            draw_to_terminal(x + x_offset + 4, y, color);
            x_offset += 7;
        } else if c == 'Z' {
            for i in 0..4 {
                draw_to_terminal(x + x_offset + i, y, color);
            }
            for i in 0..2 {
                draw_to_terminal(x + x_offset + i + 2, y + 1, color);
            }
            for i in 0..2 {
                draw_to_terminal(x + x_offset + i + 1, y + 2, color);
            }
            for i in 0..4 {
                draw_to_terminal(x + x_offset + i, y + 3, color);
            }
            x_offset += 6;
        } else if c == ' ' {
            x_offset += 3;
        } else if c == '-' {
            for i in 0..3 {
                draw_to_terminal(x + x_offset + i, y + 1, color);
            }
            x_offset += 6;
        } 
    }
}










/// Takes in a string and returns a vector of pixels that represent the string in big text.
/// 
/// DIFFERENT FROM DRAW_BIG_TEXT FUNCTION:
/// - This function returns a vector of pixels that represent the text
/// - The draw_big_text function directly draws the text to the terminal
/// 
/// Arguments:
/// 
/// * `x`: The x coordinate of the top left corner of the text
/// * `y`: y coordinate of the top left corner of the text
/// * `text`: The text to be displayed
/// * `txt_color`: The color of the text
/// 
/// Returns:
/// 
/// * `Vec<Pixel>`: A vector of pixels that represent the text
pub fn big_text(x: u16, y: u16, text: &str, txt_color: Color) -> Vec<Pixel> {
    let mut x_offset = 0;
    let mut pixels = Vec::new();
    for c in text.to_uppercase().chars() {
        if c == 'A' {
            for i in 0..3 {
                pixels.push(Pixel{x: x + x_offset, y: y + i + 1, color: txt_color});
            }
            for i in 0..3 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y, color: txt_color});
            }
            for i in 0..3 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y: y + 2, color: txt_color});
            }
            for i in 0..3 {
                pixels.push(Pixel{x: x + x_offset + 4, y: y + i + 1, color: txt_color});
            }
            x_offset += 7;
        } else if c == 'B' {
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset, y: y + i, color: txt_color});
            }
            for i in 0..3 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y, color: txt_color});
            }
            pixels.push(Pixel{x: x + x_offset + 1, y: y + 1, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 2, y: y + 1, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 3, y: y + 2, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 1, y: y + 3, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 2, y: y + 3, color: txt_color});
            x_offset += 6;
        } else if c == 'C' {
            for i in 0..2 {
                pixels.push(Pixel{x: x + x_offset, y: y + i + 1, color: txt_color});
            }
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y, color: txt_color});
            }
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y: y + 3, color: txt_color});
            }
            
            x_offset += 7;
        } else if c == 'D' {
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset, y: y + i, color: txt_color});
            }
            for i in 0..3 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y, color: txt_color});
            }
            for i in 0..3 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y: y + 3, color: txt_color});
            }
            for i in 0..2 {
                pixels.push(Pixel{x: x + x_offset + 4, y: y + 1 + i, color: txt_color});
            }

            x_offset += 7;
        } else if c == 'E' {
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset, y: y + i, color: txt_color});
            }
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y, color: txt_color});
            }
            for i in 0..2 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y: y + 1, color: txt_color});
            }
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y: y + 3, color: txt_color});
            }
            x_offset += 7;
        } else if c == 'F' {
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset, y: y + i, color: txt_color});
            }
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y, color: txt_color});
            }
            for i in 0..2 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y: y + 1, color: txt_color});
            }
            x_offset += 7;
        } else if c == 'G' {
            for i in 0..2 {
                pixels.push(Pixel{x: x + x_offset, y: y + i + 1, color: txt_color});
            }
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + i + 1, y, color: txt_color});
            }
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + i + 1, y: y + 3 , color: txt_color});
            }
            pixels.push(Pixel{x: x + x_offset + 3, y: y + 2, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 4, y: y + 2, color: txt_color});
            x_offset += 7;
        } else if c == 'H' {
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset, y: y + i, color: txt_color});
            }
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + 4, y: y + i, color: txt_color});
            }
            for i in 0..3 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y: y + 1, color: txt_color});
            }
            x_offset += 7;
        } else if c == 'I' {
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + 1, y: y + i, color: txt_color});
            }
            for i in 0..3 {
                pixels.push(Pixel{x: x + x_offset + i, y: y + 3, color: txt_color});
            }
            for i in 0..3 {
                pixels.push(Pixel{x: x + x_offset + i, y, color: txt_color});
            }
            x_offset += 5;
        } else if c == 'J' {
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y, color: txt_color});
            }
            for i in 0..3 {
                pixels.push(Pixel{x: x + x_offset + 3, y: y + i, color: txt_color});
            }
            pixels.push(Pixel{x: x + x_offset + 2, y: y + 3, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 1, y: y + 3, color: txt_color});
            pixels.push(Pixel{x: x + x_offset, y: y + 2, color: txt_color});
            x_offset += 7;
        } else if c == 'K' {
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset, y: y + i, color: txt_color});
            }
            for i in 0..2 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y: y + 1, color: txt_color});
            }
            pixels.push(Pixel{x: x + x_offset + 3, y: y + 2, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 4, y: y + 3, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 3, y: y + 1, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 4, y, color: txt_color});
            x_offset += 7;
        } else if c == 'L' {
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset, y: y + i, color: txt_color});
            }
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y: y + 3, color: txt_color});
            }
            x_offset += 7;
        }
        else if c == 'M' {
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset, y: y + i, color: txt_color});
            }
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + 4, y: y + i, color: txt_color});
            }
            pixels.push(Pixel{x: x + x_offset + 1, y: y + 1, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 1, y: y + 1, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 2, y: y + 2, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 2, y: y + 2, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 3, y: y + 1, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 3, y: y + 1, color: txt_color});
            x_offset += 7;
        } else if c == 'N' {
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset, y: y + i, color: txt_color});
            }
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + 4, y: y + i, color: txt_color});
            }
            pixels.push(Pixel{x: x + x_offset + 1, y: y + 1, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 1, y: y + 1, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 2, y: y + 2, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 2, y: y + 2, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 3, y: y + 3, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 3, y: y + 3, color: txt_color});
            x_offset += 7;
        } else if c == 'O' {
            for i in 0..3 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y, color: txt_color});
            }
            for i in 0..3 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y: y + 3, color: txt_color});
            }
            for i in 0..2 {
                pixels.push(Pixel{x: x + x_offset, y: y + i + 1, color: txt_color});
            }
            for i in 0..2 {
                pixels.push(Pixel{x: x + x_offset + 4, y: y + i + 1, color: txt_color});
            }
            x_offset += 7;
        } else if c == 'P' {
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset, y: y + i, color: txt_color});
            }
            for i in 0..2 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y, color: txt_color});
            }
            pixels.push(Pixel{x: x + x_offset + 3, y: y + 1, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 3, y: y + 1, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 1, y: y + 2, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 1, y: y + 2, color: txt_color});
            x_offset += 6;
        } else if c == 'Q' {
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y, color: txt_color});
            }
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y: y + 3, color: txt_color});
            }
            for i in 0..2 {
                pixels.push(Pixel{x: x + x_offset, y: y + i + 1, color: txt_color});
            }
            for i in 0..2 {
                pixels.push(Pixel{x: x + x_offset + 5, y: y + i + 1, color: txt_color});
            }
            pixels.push(Pixel{x: x + x_offset + 4, y: y + 2, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 4, y: y + 2, color: txt_color});
            x_offset += 8;
        }
        else if c == 'R' {
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset, y: y + i, color: txt_color});
            }
            for i in 0..2 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y, color: txt_color});
            }
            pixels.push(Pixel{x: x + x_offset + 3, y: y + 1, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 3, y: y + 1, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 2, y: y + 2, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 2, y: y + 2, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 3, y: y + 3, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 3, y: y + 3, color: txt_color});
            x_offset += 6;
        } else if c == 'S' {
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + i, y, color: txt_color});
            }
            for i in 0..2 {
                pixels.push(Pixel{x: x + x_offset + i, y: y + 1, color: txt_color});
            }
            for i in 0..2 {
                pixels.push(Pixel{x: x + x_offset + i + 2, y: y + 2, color: txt_color});
            }
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + i, y: y + 3, color: txt_color});
            }
            x_offset += 6;
        }
        else if c == 'T' {
            for i in 0..5 {
                pixels.push(Pixel{x: x + x_offset + i, y, color: txt_color});
            }
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + 2, y: y + i, color: txt_color});
            }
            x_offset += 7;
        } else if c == 'U' {
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset, y: y + i, color: txt_color});
            }
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + 4, y: y + i, color: txt_color});
            }
            for i in 0..3 {
                pixels.push(Pixel{x: x + x_offset + 1 + i, y: y + 3, color: txt_color});
            }
            x_offset += 7;
        } else if c == 'V' {
            pixels.push(Pixel{x: x + x_offset, y, color: txt_color});
            pixels.push(Pixel{x: x + x_offset, y: y + 1, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 1, y: y + 2, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 2, y: y + 3, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 3, y: y + 2, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 4, y: y + 1, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 4, y, color: txt_color});
            x_offset += 7;
        } else if c == 'W' {
            // first v
            pixels.push(Pixel{x: x + x_offset, y, color: txt_color});
            pixels.push(Pixel{x: x + x_offset, y: y + 1, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 1, y: y + 2, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 2, y: y + 3, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 3, y: y + 2, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 4, y: y + 1, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 4, y, color: txt_color});

            // second v
            pixels.push(Pixel{x: x + x_offset + 4, y, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 4, y: y + 1, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 5, y: y + 2, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 6, y: y + 3, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 7, y: y + 2, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 8, y: y + 1, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 8, y, color: txt_color});
            x_offset += 11
        } else if c == 'X' {
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + i, y: y + i, color: txt_color});
            }
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + i, y: y + 3 - i, color: txt_color});
            }
            x_offset += 6;
        } else if c == 'Y' {
            for i in 0..3 {
                pixels.push(Pixel{x: x + x_offset + i, y: y + i, color: txt_color});
            }
            pixels.push(Pixel{x: x + x_offset + 2, y: y + 3, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 3, y: y + 1, color: txt_color});
            pixels.push(Pixel{x: x + x_offset + 4, y, color: txt_color});
            x_offset += 7;
        } else if c == 'Z' {
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + i, y, color: txt_color});
            }
            for i in 0..2 {
                pixels.push(Pixel{x: x + x_offset + i + 2, y: y + 1, color: txt_color});
            }
            for i in 0..2 {
                pixels.push(Pixel{x: x + x_offset + i + 1, y: y + 2, color: txt_color});
            }
            for i in 0..4 {
                pixels.push(Pixel{x: x + x_offset + i, y: y + 3, color: txt_color});
            }
            x_offset += 6;
        } else if c == ' ' {
            x_offset += 3;
        } else if c == '-' {
            for i in 0..3 {
                pixels.push(Pixel{x: x + x_offset + i, y: y + 1, color: txt_color});
            }
            x_offset += 6;
        } 
    }
    pixels
}