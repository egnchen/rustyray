//! ppm: library for PPM file read & write.

use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;

use image::ImageBuffer;

use crate::io::Color24;
use crate::utils::{Picture, Vec3};

/// write_to_ppm: Write a picture to PPM file
pub fn write_to_ppm(p: &Picture, filename: &str) -> io::Result<()> {
    let f = File::create(filename)?;
    let mut stream = BufWriter::new(f);

    writeln!(&mut stream, "P3")?;
    writeln!(&mut stream, "{} {}", p.width, p.height)?;
    writeln!(&mut stream, "255")?;
    assert_eq!(p.data.len(), (p.height * p.width) as usize);
    let mut v: usize = 0;
    for _i in 0..p.width {
        for _j in 0..p.height {
            let c = &p.data[v];
            let c = Color24::from(c);
            write!(&mut stream, " {} {} {}", c.0, c.1, c.2)?;
            v += 1;
        }
        write!(&mut stream, "\n")?;
    }
    stream.flush()
}

/// write_to_png: Write picture to png file
pub fn write_to_png(p: &Picture, filename: &str) {
    let w = p.width as u32;
    let h = p.height as u32;
    let buf = ImageBuffer::from_fn(w, h, |x, y| {
        let v = (y * w + x) as usize;
        let c = Color24::from(&p.data[v]);
        image::Rgb([c.0, c.1, c.2])
    });
    buf.save(filename).unwrap();
}
