use crate::rendering::{Pixel, erase_pixel, draw_to_terminal};


/// An object is a collection of pixels that make up a shape, it can be moved, deleted and drawn to the terminal.\
/// It can also be used to represent a single pixel.
/// 
/// Properties:
/// 
/// * `pixels`: A vector of pixels that make up the object.
pub struct Object {
    pub pixels: Vec<Pixel>
}

impl Object {
    /// Creates a new `Object` with the given pixels.
    /// 
    /// Arguments:
    /// 
    /// * `pixels`: A vector of pixels that make up the object.
    /// 
    /// Returns:
    /// 
    /// * `Object`: A new `Object` with the given pixels.
    pub fn new(pixels: Vec<Pixel>) -> Object {
        Object {
            pixels
        }
    }
    /// Draws the object to the terminal.
    /// 
    /// Arguments:
    /// 
    /// * `self`: A reference to the `Object` to draw.
    pub fn draw(&self) {
        for pixel in &self.pixels {
            draw_to_terminal(pixel.x, pixel.y, pixel.color);
        }
    }
    /// Moves an object by the given amount of pixels.\
    /// (object is erased before being moved, and then drawn again)
    /// 
    /// Arguments:
    /// 
    /// * `self`: A mutable reference to the `Object` to move.
    /// * `x`: The amount of pixels to move the object in the x direction.
    /// * `y`: The y coordinate of the pixel
    pub fn move_object(&mut self, x: i32, y: i32) {
        for pixel in &self.pixels {
            erase_pixel(pixel.x, pixel.y);
        }
        for pixel in &mut self.pixels {
            pixel.x = (pixel.x as i32 + x) as u16;
            pixel.y = (pixel.y as i32 + y) as u16;
        }
        self.draw();
    }
    /// Erases all the pixels in the `pixels` vector - useful for erasing an object without deleting it.
    /// 
    /// Arguments:
    /// 
    /// * `self`: A mutable reference to the `Object` to erase.
    pub fn erase_pixels(&mut self) {
        for pixel in &self.pixels {
            erase_pixel(pixel.x, pixel.y);
        }
        self.pixels = vec![];
    }
    /// Adds pixels to the object.
    /// 
    /// Arguments:
    /// 
    /// * `self`: A mutable reference to the `Object` to add pixels to.
    /// * `pixels`: A vector of pixels to add to the object.
    pub fn add_pixels(&mut self, pixels: Vec<Pixel>) {
        self.pixels.extend(pixels);
    }
    /// Erases each pixel of this object from the terminal and drops this object from memory. \
    /// 
    /// Arguments:
    /// 
    /// * `self`: A reference to the `Object` to delete.
    pub fn delete(self) {
        for pixel in &self.pixels {
            erase_pixel(pixel.x, pixel.y);
        }
        drop(self)
    }
}