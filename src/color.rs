pub use crate::vec3::Vec3 as Color;
use std::io::Write;

pub fn write_color(file: &mut std::fs::File, color: Color) {
  let ir = (color.x() * 255.999) as u32;
  let ig = (color.y() * 255.999) as u32;
  let ib = (color.z() * 255.999) as u32;
  let line = format!("{ir} {ig} {ib}\n");
  match file.write(line.as_bytes()) {
    Ok(_) => {}
    Err(error) => panic!("Problem writing the file: {:?}", error),
  };
}
