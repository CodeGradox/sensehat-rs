use framebuffer::{Framebuffer, FramebufferError};
use glob::{glob, GlobError, PatternError};

use std::fmt;

/// A rgb888 color pixel.
///
/// A pixel on the sensehat LED matrix is actually a hex565.
/// That means a pixel is 16-bit instead of 24-bit.
/// (5 for red, 6 for green, 5 for blue, 5+6+5=16)
pub type Pixel = (u8, u8, u8);

/// The image orientation.
/// 0°, 90°, 180°, 270°
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Orientation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

/// Represents the LED matrix.
pub struct Display {
    framebuffer: Framebuffer,
    frame: [u8; 128],
    orientation: Orientation,
}

/// The errors which can occur when using the display.
#[derive(Debug)]
pub enum DisplayError {
    MissingFramebuffer,
    GlobError(GlobError),
    PatternError(PatternError),
    FramebufferError(FramebufferError),
}

impl Display {
    /// Try to create a new Display object.
    ///
    /// Will open the sensehat framebuffer and map it to memory.
    pub fn new() -> Result<Self, DisplayError> {
        // The id of the sensehat framebuffer
        let rpi_sense_fb = b"RPi-Sense FB";
        
        // Iterator for framebuffers located in /dev
        let path = glob("/dev/fb*")?;

        // Locates the sensehat framebuffer
        let framebuffer = path.filter_map(Result::ok)
            .filter_map(|file| Framebuffer::new(&file.to_string_lossy()).ok())
            .filter(|fb| {
                let id = fb.fix_screen_info.id;
                rpi_sense_fb[..] == id[..rpi_sense_fb.len()]
            })
            .next();
        match framebuffer {
            Some(fb) => Ok(Self {
                framebuffer: fb,
                frame: [0; 128],
                orientation: Orientation::Deg0,
                }),
            None => Err(DisplayError::MissingFramebuffer),
        }
    }

    /// Sets the orientation of the display. The default orientation is with
    /// the HDMI port facing downwards on the Raspberry Pi 3 model B.
    pub fn set_rotation(&mut self, ori: Orientation, redraw: bool) {
        self.orientation = ori;
        if redraw {
            self.draw();
        }
    }

    /// Updates the entire LED matrix based on a 64 length array of pixel values.
    /// A pixel is a triplet of u8's (red, green, blue).
    pub fn set_pixels(&mut self, pixel_list: &[Pixel; 64]) {
        for (pos, pixel) in (0..64).map(|x| x * 2).zip(pixel_list.iter()) {
            let (msb, lsb) = convert_pixel(*pixel);
            self.frame[pos] = lsb;
            self.frame[pos + 1] = msb;
        }
        self.draw();
    }

    /// Sets a single LED matrix pixel at the given (x, y) coordinate
    /// to the given color.
    pub fn set_pixel(&mut self, x: usize, y: usize, p: Pixel) {
        // TODO: return an error if x or y are out of bounds?
        if x > 7 || y > 7 { return; }
        let pos = 2 * (x + 8 * y);
        let (msb, lsb) = convert_pixel(p);
        self.frame[pos] = lsb;
        self.frame[pos + 1] = msb;
        self.draw();
    }

    /// Rotates the LED matrix display based on the orientation.
    fn draw(&mut self) {
        if self.orientation == Orientation::Deg0 {
            // No need to flip the image as this is the default orientation.
            self.framebuffer.write_frame(&self.frame);
        } else {
            let mut temp = [0; 128];
            let mut i = 0;
            for y in 0..8 {
                for x in 0..8 {
                    let cor = self.rotation_func(x, y);
                    temp[cor] = self.frame[i];
                    temp[cor + 1] = self.frame[i + 1];
                    i += 2;
                }
            }
            self.framebuffer.write_frame(&temp);
        }
    }

    /// Sets the entire LED matrix to a single color, defaults to blank/off.
    pub fn clear(&mut self, color: Option<Pixel>) {
        match color {
            Some(c) => {
                let (msb, lsb) = convert_pixel(c);
                for pos in (0..64).map(|x| x * 2) {
                    self.frame[pos] = lsb;
                    self.frame[pos + 1] = msb;
                }
            }
            None => {
                for p in self.frame.iter_mut() { *p = 0 }
            }
        }
        self.framebuffer.write_frame(&self.frame);
    }

    /// Helper function for mapping a (x, y) coordinate on the
    /// 2D LED matrix to a 1D position on the frame.
    /// A pixel in the frame is actually 16-bit, but since we can
    /// only write to the framebuffer with u8 slices, we have to
    /// split up each pixel in two. This function returns the position
    /// of the 8 MSB of a pixel.
    fn rotation_func(&self, x: usize, y: usize) -> usize {
        use self::Orientation::*;
        match self.orientation {
            Deg0 => 2 * (x + 8 * y),
            Deg90 => 2 * ((7 - y) + 8 * x),
            Deg180 => 126 - 2 * (x + 8 * y),
            Deg270 => 2 * (y + 8 * (7 - x)),
        }
    }
}

/// Converts a rgb888 pixel into a rgb565 pixel
fn convert_pixel(p: Pixel) -> (u8, u8) {
    let r = p.0 & 0xF8;
    let g = p.1 >> 2;
    let b = p.2 >> 3;
    (r | (g >> 3), (g << 5) | b)
}

impl fmt::Debug for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display {{ framebuffer: {:?} orientation: {:?} }}",
            self.framebuffer,
            self.orientation)
    }
}

impl From<GlobError> for DisplayError {
    fn from(err: GlobError) -> Self {
        DisplayError::GlobError(err)
    }
}

impl From<PatternError> for DisplayError {
    fn from(err: PatternError) -> Self {
        DisplayError::PatternError(err)
    }
}

impl From<FramebufferError> for DisplayError {
    fn from(err: FramebufferError) -> Self {
        DisplayError::FramebufferError(err)
    }
}
