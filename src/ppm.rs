//! ppm: library for PPM file read & write.

use std::io;
use std::io::Write;
use std::fs::File;
use std::io::{BufWriter, BufReader};

use crate::vec::Vec3;

/// Color in RGB
pub type Color = Vec3<u8>;

pub struct Picture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Color>,
}

impl Picture {

    /// read_from_file: Read PPM picture from file
    /// TODO
    pub fn read_from_file(filename: &str) -> Result<Picture, ()> {
        // -- pass
        Ok(Picture {
            width: 0,
            height: 0,
            data: vec![],
        })
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
        for _i in 1..=self.width {
            for _j in 1..=self.height {
                let c = &self.data[v];
                write!(&mut stream, " {} {} {}", c.0, c.1, c.2)?;
                v += 1;
            }
            write!(&mut stream, "\n")?;
        }
        stream.flush()
    }
}
