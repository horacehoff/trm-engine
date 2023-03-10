use crate::{rendering::draw, Pixel};


fn draw_t(buffer: &Vec<Pixel>, x: u16, y: u16) -> Vec<Pixel> {
    let mut out_buffer: Vec<Pixel> = buffer.clone();

    for i in 0..5 {
        draw(&mut out_buffer, x + i, y, "blue");
    }
    for i in 0..4 {
        draw(&mut out_buffer, x + 2, y + i, "blue");
    }

    out_buffer
}



#[inline(always)]
pub fn draw_text<'a>(buffer: &'a mut Vec<Pixel>, x: u16, y: u16, text: &str, color: &str) -> &'a Vec<Pixel> {
    let mut x_offset = 0;
    for (i, c) in text.to_uppercase().chars().enumerate() {
        if c == 'A' {
            for i in 0..3 {
                draw(buffer, x + x_offset, y + i + 1, color);
            }
            for i in 0..3 {
                draw(buffer, x + x_offset + 1 + i, y, color);
            }
            for i in 0..3 {
                draw(buffer, x + x_offset + 1 + i, y + 2, color);
            }
            for i in 0..3 {
                draw(buffer, x + x_offset + 4, y + i + 1, color);
            }
            x_offset += 7;
        } else if c == 'B' {
            for i in 0..4 {
                draw(buffer, x + x_offset, y + i, color);
            }
            for i in 0..3 {
                draw(buffer, x + x_offset + 1 + i, y, color);
            }
            draw(buffer, x + x_offset + 1, y + 1, color);
            draw(buffer, x + x_offset + 2, y + 1, color);
            draw(buffer, x + x_offset + 3, y + 2, color);
            draw(buffer, x + x_offset + 1, y + 3, color);
            draw(buffer, x + x_offset + 2, y + 3, color);
            x_offset += 6;
        } else if c == 'C' {
            for i in 0..2 {
                draw(buffer, x + x_offset, y + i + 1, color);
            }
            for i in 0..4 {
                draw(buffer, x + x_offset + 1 + i, y, color);
            }
            for i in 0..4 {
                draw(buffer, x + x_offset + 1 + i, y + 3, color);
            }
            
            x_offset += 7;
        } else if c == 'D' {
            for i in 0..4 {
                draw(buffer, x + x_offset, y + i, color);
            }
            for i in 0..3 {
                draw(buffer, x + x_offset + 1 + i, y, color);
            }
            for i in 0..3 {
                draw(buffer, x + x_offset + 1 + i, y + 3, color);
            }
            for i in 0..2 {
                draw(buffer, x + x_offset + 4, y + 1 + i, color);
            }

            x_offset += 7;
        } else if c == 'E' {
            for i in 0..4 {
                draw(buffer, x + x_offset, y + i, color);
            }
            for i in 0..4 {
                draw(buffer, x + x_offset + 1 + i, y, color);
            }
            for i in 0..2 {
                draw(buffer, x + x_offset + 1 + i, y + 1, color);
            }
            for i in 0..4 {
                draw(buffer, x + x_offset + 1 + i, y + 3, color);
            }
            x_offset += 7;
        } else if c == 'F' {
            for i in 0..4 {
                draw(buffer, x + x_offset, y + i, color);
            }
            for i in 0..4 {
                draw(buffer, x + x_offset + 1 + i, y, color);
            }
            for i in 0..2 {
                draw(buffer, x + x_offset + 1 + i, y + 1, color);
            }
            x_offset += 7;
        } else if c == 'G' {
            for i in 0..2 {
                draw(buffer, x + x_offset, y + i + 1, color);
            }
            for i in 0..4 {
                draw(buffer, x + x_offset + i + 1, y, color);
            }
            for i in 0..4 {
                draw(buffer, x + x_offset + i + 1, y + 3 , color);
            }
            draw(buffer, x + x_offset + 3, y + 2, color);
            draw(buffer, x + x_offset + 4, y + 2, color);
            x_offset += 7;
        } else if c == 'H' {
            for i in 0..4 {
                draw(buffer, x + x_offset, y + i, color);
            }
            for i in 0..4 {
                draw(buffer, x + x_offset + 4, y + i, color);
            }
            for i in 0..3 {
                draw(buffer, x + x_offset + 1 + i, y + 1, color);
            }
            x_offset += 7;
        } else if c == 'I' {
            for i in 0..4 {
                draw(buffer, x + x_offset + 1, y + i, color);
            }
            for i in 0..3 {
                draw(buffer, x + x_offset + i, y + 3, color);
            }
            for i in 0..3 {
                draw(buffer, x + x_offset + i, y, color);
            }
            x_offset += 5;
        } else if c == 'J' {
            for i in 0..4 {
                draw(buffer, x + x_offset + 1 + i, y, color);
            }
            for i in 0..3 {
                draw(buffer, x + x_offset + 3, y + i, color);
            }
            draw(buffer, x + x_offset + 2, y + 3, color);
            draw(buffer, x + x_offset + 1, y + 3, color);
            draw(buffer, x + x_offset, y + 2, color);
            x_offset += 7;
        } else if c == 'K' {
            for i in 0..4 {
                draw(buffer, x + x_offset, y + i, color);
            }
            for i in 0..2 {
                draw(buffer, x + x_offset + 1 + i, y + 1, color);
            }
            draw(buffer, x + x_offset + 3, y + 2, color);
            draw(buffer, x + x_offset + 4, y + 3, color);
            draw(buffer, x + x_offset + 3, y + 1, color);
            draw(buffer, x + x_offset + 4, y, color);
            x_offset += 7;
        } else if c == 'L' {
            for i in 0..4 {
                draw(buffer, x + x_offset, y + i, color);
            }
            for i in 0..4 {
                draw(buffer, x + x_offset + 1 + i, y + 3, color);
            }
            x_offset += 7;
        }
        else if c == 'M' {
            for i in 0..4 {
                draw(buffer, x + x_offset, y + i, color);
            }
            for i in 0..4 {
                draw(buffer, x + x_offset + 4, y + i, color);
            }
            draw(buffer, x + x_offset + 1, y + 1, color);
            draw(buffer, x + x_offset + 1, y + 1, color);
            draw(buffer, x + x_offset + 2, y + 2, color);
            draw(buffer, x + x_offset + 2, y + 2, color);
            draw(buffer, x + x_offset + 3, y + 1, color);
            draw(buffer, x + x_offset + 3, y + 1, color);
            x_offset += 7;
        } else if c == 'N' {
            for i in 0..4 {
                draw(buffer, x + x_offset, y + i, color);
            }
            for i in 0..4 {
                draw(buffer, x + x_offset + 4, y + i, color);
            }
            draw(buffer, x + x_offset + 1, y + 1, color);
            draw(buffer, x + x_offset + 1, y + 1, color);
            draw(buffer, x + x_offset + 2, y + 2, color);
            draw(buffer, x + x_offset + 2, y + 2, color);
            draw(buffer, x + x_offset + 3, y + 3, color);
            draw(buffer, x + x_offset + 3, y + 3, color);
            x_offset += 7;
        } else if c == 'O' {
            for i in 0..3 {
                draw(buffer, x + x_offset + 1 + i, y, color);
            }
            for i in 0..3 {
                draw(buffer, x + x_offset + 1 + i, y + 3, color);
            }
            for i in 0..2 {
                draw(buffer, x + x_offset, y + i + 1, color);
            }
            for i in 0..2 {
                draw(buffer, x + x_offset + 4, y + i + 1, color);
            }
            x_offset += 7;
        } else if c == 'P' {
            for i in 0..4 {
                draw(buffer, x + x_offset, y + i, color);
            }
            for i in 0..2 {
                draw(buffer, x + x_offset + 1 + i, y, color);
            }
            draw(buffer, x + x_offset + 3, y + 1, color);
            draw(buffer, x + x_offset + 3, y + 1, color);
            draw(buffer, x + x_offset + 1, y + 2, color);
            draw(buffer, x + x_offset + 1, y + 2, color);
            x_offset += 6;
        } else if c == 'Q' {
            for i in 0..4 {
                draw(buffer, x + x_offset + 1 + i, y, color);
            }
            for i in 0..4 {
                draw(buffer, x + x_offset + 1 + i, y + 3, color);
            }
            for i in 0..2 {
                draw(buffer, x + x_offset, y + i + 1, color);
            }
            for i in 0..2 {
                draw(buffer, x + x_offset + 5, y + i + 1, color);
            }
            draw(buffer, x + x_offset + 4, y + 2, color);
            draw(buffer, x + x_offset + 4, y + 2, color);
            x_offset += 8;
        }
        else if c == 'R' {
            for i in 0..4 {
                draw(buffer, x + x_offset, y + i, color);
            }
            for i in 0..2 {
                draw(buffer, x + x_offset + 1 + i, y, color);
            }
            draw(buffer, x + x_offset + 3, y + 1, color);
            draw(buffer, x + x_offset + 3, y + 1, color);
            draw(buffer, x + x_offset + 2, y + 2, color);
            draw(buffer, x + x_offset + 2, y + 2, color);
            draw(buffer, x + x_offset + 3, y + 3, color);
            draw(buffer, x + x_offset + 3, y + 3, color);
            x_offset += 6;
        } else if c == 'S' {
            for i in 0..4 {
                draw(buffer, x + x_offset + i, y, color);
            }
            for i in 0..2 {
                draw(buffer, x + x_offset + i, y + 1, color);
            }
            for i in 0..2 {
                draw(buffer, x + x_offset + i + 2, y + 2, color);
            }
            for i in 0..4 {
                draw(buffer, x + x_offset + i, y + 3, color);
            }
            x_offset += 6;
        }
        else if c == 'T' {
            for i in 0..5 {
                draw(buffer, x + x_offset + i, y, color);
            }
            for i in 0..4 {
                draw(buffer, x + x_offset + 2, y + i, color);
            }
            x_offset += 7;
        } else if c == 'U' {
            for i in 0..4 {
                draw(buffer, x + x_offset, y + i, color);
            }
            for i in 0..4 {
                draw(buffer, x + x_offset + 4, y + i, color);
            }
            for i in 0..3 {
                draw(buffer, x + x_offset + 1 + i, y + 3, color);
            }
            x_offset += 7;
        } else if c == 'V' {
            draw(buffer, x + x_offset, y, color);
            draw(buffer, x + x_offset, y + 1, color);
            draw(buffer, x + x_offset + 1, y + 2, color);
            draw(buffer, x + x_offset + 2, y + 3, color);
            draw(buffer, x + x_offset + 3, y + 2, color);
            draw(buffer, x + x_offset + 4, y + 1, color);
            draw(buffer, x + x_offset + 4, y, color);
            x_offset += 7;
        } else if c == 'W' {
            // first v
            draw(buffer, x + x_offset, y, color);
            draw(buffer, x + x_offset, y + 1, color);
            draw(buffer, x + x_offset + 1, y + 2, color);
            draw(buffer, x + x_offset + 2, y + 3, color);
            draw(buffer, x + x_offset + 3, y + 2, color);
            draw(buffer, x + x_offset + 4, y + 1, color);
            draw(buffer, x + x_offset + 4, y, color);

            // second v
            draw(buffer, x + x_offset + 4, y, color);
            draw(buffer, x + x_offset + 4, y + 1, color);
            draw(buffer, x + x_offset + 5, y + 2, color);
            draw(buffer, x + x_offset + 6, y + 3, color);
            draw(buffer, x + x_offset + 7, y + 2, color);
            draw(buffer, x + x_offset + 8, y + 1, color);
            draw(buffer, x + x_offset + 8, y, color);
            x_offset += 11
        } else if c == 'X' {
            for i in 0..4 {
                draw(buffer, x + x_offset + i, y + i, color);
            }
            for i in 0..4 {
                draw(buffer, x + x_offset + i, y + 3 - i, color);
            }
            x_offset += 6;
        } else if c == 'Y' {
            for i in 0..3 {
                draw(buffer, x + x_offset + i, y + i, color);
            }
            draw(buffer, x + x_offset + 2, y + 3, color);
            draw(buffer, x + x_offset + 3, y + 1, color);
            draw(buffer, x + x_offset + 4, y, color);
            x_offset += 7;
        } else if c == 'Z' {
            for i in 0..4 {
                draw(buffer, x + x_offset + i, y, color);
            }
            for i in 0..2 {
                draw(buffer, x + x_offset + i + 2, y + 1, color);
            }
            for i in 0..2 {
                draw(buffer, x + x_offset + i + 1, y + 2, color);
            }
            for i in 0..4 {
                draw(buffer, x + x_offset + i, y + 3, color);
            }
            x_offset += 6;
        } else if c == ' ' {
            x_offset += 3;
        } else if c == '-' {
            for i in 0..3 {
                draw(buffer, x + x_offset + i, y + 1, color);
            }
            x_offset += 6;
        } 
    }
    buffer
}