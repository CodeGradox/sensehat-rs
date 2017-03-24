use {SenseHatError, SenseHatResult};

use libc::{ioctl, c_ulong};
use framebuffer::Framebuffer;
use byteorder::{ByteOrder, LittleEndian};
use glob::glob;

use std::fmt;
use std::os::unix::io::AsRawFd;

const SENSE_HAT_FBIOGET_GAMMA: c_ulong = 61696;
const SENSE_HAT_FBIOSET_GAMMA: c_ulong = 61697;
const SENSE_HAT_FBIORESET_GAMMA: c_ulong = 61698;
const SENSE_HAT_GAMMA_DEFAULT: c_ulong = 0;
const SENSE_HAT_GAMMA_LOW: c_ulong = 1;

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

impl Display {
    /// Try to create a new Display object.
    ///
    /// Will open the sensehat framebuffer and map it to memory.
    pub fn new() -> SenseHatResult<Self> {
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
            None => Err(SenseHatError::MissingFramebuffer),
        }
    }

    /// Helper function.
    ///
    /// Rotates and draws the LED matrix display based on the orientation.
    fn draw(&mut self) {
        if self.orientation == Orientation::Deg0 {
            self.framebuffer.write_frame(&self.frame);
        } else {
            let mut temp = [0; 128];
            let mut i = 0;
            for y in 0..8 {
                for x in 0..8 {
                    let cor = self.map_position(x, y);
                    let pixel = LittleEndian::read_u16(&self.frame[i..]);
                    LittleEndian::write_u16(&mut temp[cor..], pixel);
                    i += 2;
                }
            }
            self.framebuffer.write_frame(&temp);
        }
    }

    /// Helper function.
    ///
    /// Function for mapping a (x, y) coordinate on the
    /// 2D LED matrix to a 1D position on the frame.
    /// A pixel in the frame is actually 16-bit, but since we can
    /// only write to the framebuffer with u8 slices, we have to
    /// split up each pixel in two. This function returns the position
    /// of the 8 MSB of a pixel.
    fn map_position(&self, x: usize, y: usize) -> usize {
        use self::Orientation::*;
        match self.orientation {
            Deg0 => 2 * (x + 8 * y),
            Deg90 => 2 * ((7 - y) + 8 * x),
            Deg180 => 126 - 2 * (x + 8 * y),
            Deg270 => 2 * (y + 8 * (7 - x)),
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

    /// Flips the pixels on the LED matrix horizontaly.
    /// Returns a list of the LED pixels.
    pub fn flip_h(&mut self, redraw: bool) -> [Pixel; 64] {
        let mut pixels = self.get_pixels();
        for slice in pixels[..].chunks_mut(8) {
            slice.reverse();
        }
        if redraw {
            self.set_pixels(&pixels);   
        }
        pixels
    }

    /// Flips the pixels on the LED matrix vertically.
    /// Returns a list of the LED pixels.
    pub fn flip_v(&mut self, redraw: bool) -> [Pixel; 64] {
        let mut pixels = self.get_pixels();
        for i in 0..8 {
            for j in 0..4 {
                let offset = j * 8;
                pixels.swap(i + offset, i + 56 - offset);
            }
        }
        if redraw {
            self.set_pixels(&pixels);
        }
        pixels
    }

    /// Updates the entire LED matrix based on a 64 length array of pixel values.
    /// A pixel is a triplet of u8's (red, green, blue).
    pub fn set_pixels(&mut self, pixels: &[Pixel; 64]) {
        for (pos, pixel) in self.frame[..]
            .chunks_mut(2)
            .zip(pixels.iter()
                       .map(|&p| convert_from_pixel(p)))
        {
            LittleEndian::write_u16(pos, pixel);
        }
        self.draw();
    }

    /// Get a vector of all `Pixel`s on the currently displayed image.
    pub fn get_pixels(&self) -> [Pixel; 64] {
        let mut pixels = [(0, 0, 0); 64];
        for (index, value) in pixels
            .iter_mut()
            .zip(self.frame[..]
                     .chunks(2)
                     .map(LittleEndian::read_u16)
                     .map(convert_to_pixel))
        {
            *index = value;
        }
        pixels
    }

    /// Sets a single LED matrix pixel at the given (x, y) coordinate
    /// to the given color.
    /// Returns an error if the coordinates are out of bounds.
    pub fn set_pixel(&mut self, x: usize, y: usize, p: Pixel) -> SenseHatResult<()> {
        if x > 7 || y > 7 {
            return Err(SenseHatError::OutOfBounds);
        }
        let pos = 2 * (x + 8 * y);
        let pixel = convert_from_pixel(p);
        LittleEndian::write_u16(&mut self.frame[pos..], pixel);
        self.draw();
        Ok(())
    }

    /// Returns a single pixel value at the given coordinate.
    /// Returns an error if the coordinates are out of bounds.
    pub fn get_pixel(&self, x: usize, y: usize) -> SenseHatResult<Pixel> {
        if x > 7 || y > 7 {
            return Err(SenseHatError::OutOfBounds);
        }
        let pos = self.map_position(x, y);
        let value = LittleEndian::read_u16(&self.frame[pos..]);
        let pixel = convert_to_pixel(value);
        Ok(pixel)
    }

    /// Sets the entire LED matrix to a single color, defaults to blank/off.
    pub fn clear(&mut self, color: Option<Pixel>) {
        match color {
            Some(c) => {
                let pixel = convert_from_pixel(c);
                for pos in self.frame[..].chunks_mut(2) {
                    LittleEndian::write_u16(pos, pixel);
                }
            }
            None => {
                for p in self.frame.iter_mut() { *p = 0 }
            }
        }
        self.framebuffer.write_frame(&self.frame);
    }

    /// Retuns the current gamma settings.
    pub fn gamma(&self) -> [u8; 32] {
        let mut buffer = [0u8; 32];
        unsafe {
            let fd = self.framebuffer.device.as_raw_fd();
            ioctl(fd, SENSE_HAT_FBIOGET_GAMMA, &mut buffer);
            // TODO: Maybe check ioctl return value for errors.
        }
        buffer
    }

    /// Changes the gamma settings.
    pub fn set_gamma(&mut self, buffer: &[u8; 32]) -> SenseHatResult<()> {
        if !buffer.iter().all(|&x| x <= 31) {
            return Err(SenseHatError::InvalidGamma);
        }
        unsafe {
            let fd = self.framebuffer.device.as_raw_fd();
            ioctl(fd, SENSE_HAT_FBIOSET_GAMMA, buffer);
            // TODO: Maybe check ioctl return value for errors.
        }
        Ok(())
    }

    /// Resets the LED matrix gamma correction to default.
    pub fn reset_gamma(&mut self) {
        unsafe {
            let fd = self.framebuffer.device.as_raw_fd();
            ioctl(fd, SENSE_HAT_FBIORESET_GAMMA, SENSE_HAT_GAMMA_DEFAULT);
            // TODO: Maybe check ioctl return value for errors.
        }
    }

    /// Checks if the display is set to low light mode.
    pub fn is_low_light(&self) -> bool {
        let low: [u8; 32] = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 10, 10];
        let cur_gamma = self.gamma();
        cur_gamma == low
    }

    /// Enables or disables low light mode.
    pub fn low_light(&mut self, set_low: bool) {
        unsafe {
            let fd = self.framebuffer.device.as_raw_fd();
            let cmd = if set_low { SENSE_HAT_GAMMA_LOW } else { SENSE_HAT_GAMMA_DEFAULT };
            ioctl(fd, SENSE_HAT_FBIORESET_GAMMA, cmd);
        }
    }
}

/// Converts a rgb888 pixel into a rgb565 pixel.
fn convert_from_pixel(p: Pixel) -> u16 {
    let r = (p.0 >> 3) as u16;
    let g = (p.1 >> 2) as u16;
    let b = (p.2 >> 3) as u16;
    (r << 11) | (g << 5) | b
}

/// Converts a rgb565 pixel to a rgb888 pixel.
fn convert_to_pixel(val: u16) -> Pixel {
    let (msb, lsb) = ((val >> 8) as u8, val as u8);
    let r = msb & 0xF8;
    let g = ((msb & 0x07) << 3) | (lsb & 0xE0);
    let b = lsb & 0x1F;
    (r, g << 2, b << 3)
}

impl fmt::Debug for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display {{ framebuffer: {:?} orientation: {:?} }}",
            self.framebuffer,
            self.orientation)
    }
}
