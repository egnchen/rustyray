//! ppm: library for PPM file read & write.

use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;

use image::{open, ImageBuffer, RgbImage};

use crate::io::Color24;
use crate::utils::{Color, Picture};

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
            write!(&mut stream, " {} {} {}", c.x, c.y, c.z)?;
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
        image::Rgb([c.x, c.y, c.z])
    });
    buf.save(filename).unwrap();
}

/// read image file of arbitrary type
pub fn read_picture(filename: &str) -> Picture {
    let buf: RgbImage = open(filename).expect("Failed to read image.").into_rgb();
    Picture {
        width: buf.width() as usize,
        height: buf.height() as usize,
        data: buf
            .pixels()
            .map(|x| {
                Color::new(
                    x.0[0] as f32 / 255.0,
                    x.0[1] as f32 / 255.0,
                    x.0[2] as f32 / 255.0,
                )
            })
            .collect(),
    }
}
