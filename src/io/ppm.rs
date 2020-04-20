//! ppm: library for PPM file read & write.

use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;

use crate::utils::Vec3;

/// Color in RGB
pub type Color = Vec3<f64>;
pub type Color24 = Vec3<u8>;

impl Color24 {
    pub fn from(c: &Color) -> Self {
        Color24 {
            0: (c.0 * 255.0) as u8,
            1: (c.1 * 255.0) as u8,
            2: (c.2 * 255.0) as u8,
        }
    }
}

pub struct Picture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Color>,
}

impl Picture {
    pub fn new(width: u32, height: u32) -> Picture {
        Picture {
            width,
            height,
            data: vec![Vec3(0.0, 0.0, 0.0); (width * height) as usize],
        }
    }

    /// write_to_file: Write a picture to PPM file
    pub fn write_to_file(&self, filename: &str) -> io::Result<()> {
        let f = File::create(filename)?;
        let mut stream = BufWriter::new(f);

        writeln!(&mut stream, "P3")?;
        writeln!(&mut stream, "{} {}", self.width, self.height)?;
        writeln!(&mut stream, "255")?;
        assert_eq!(self.data.len(), (self.height * self.width) as usize);
        let mut v: usize = 0;
        for _i in 0..self.width {
            for _j in 0..self.height {
                let c = &self.data[v];
                let c = Color24::from(c);
                write!(&mut stream, " {} {} {}", c.0, c.1, c.2)?;
                v += 1;
            }
            write!(&mut stream, "\n")?;
        }
        stream.flush()
    }
}
