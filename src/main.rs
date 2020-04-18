use ray_tracer::ppm::{Picture};
use ray_tracer::vec::Vec3;

fn main() {
    let mut p = Picture {
        width: 200,
        height: 200,
        data: vec![]
    };
    for i in 1..=p.width {
        for j in 1..=p.height {
            let v = (i as i32 - j as i32).abs() as u8;
            p.data.push(Vec3 {
                0: v,
                1: v ^ 255,
                2: v | 170
            });
        }
    }
    p.write_to_file("out.ppm").expect("Failed to write file.");
}
