use framebuffer::{Framebuffer, FramebufferError};
use glob::{glob, GlobError, PatternError};

use std::fmt;

/// A rgb888 color pixel
pub type Pixel = (u8, u8, u8);

// #[derive(Debug)]
/// Represents the LED matrix.
pub struct Display {
    framebuffer: Framebuffer,
    frame: [u8; 128],
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
            Some(fb) => Ok(Self { framebuffer: fb, frame: [0; 128] }),
            None => Err(DisplayError::MissingFramebuffer),
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
        self.framebuffer.write_frame(&self.frame);
    }

    /// Sets a single LED matrix pixel at the specified (x, y) coordinate
    /// to the spesific color.
    pub fn set_pixel(&mut self, pos: (usize, usize), p: Pixel) {
        if pos.0 > 7 || pos.1 > 7 { return; }
        let cor = 2 * (pos.0 + 8 * pos.1);
        let (msb, lsb) = convert_pixel(p);
        self.frame[cor] = lsb;
        self.frame[cor + 1] = msb;
        self.framebuffer.write_frame(&self.frame);
    }

    /// Clears the entire LED matrix by turning off all LEDs.
    pub fn clear(&mut self) {
        for p in self.frame.iter_mut() { *p = 0 }
        self.framebuffer.write_frame(&self.frame);
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
        write!(f, "Display {{ framebuffer: {:?} }}", self.framebuffer)
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
