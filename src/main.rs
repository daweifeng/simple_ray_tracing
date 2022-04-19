use std::fs::File;
use std::io::Write;

mod vec3;

use vec3::Vec3 as Point; // 3D point
use vec3::Vec3 as Color; // RGB color

fn main() -> std::io::Result<()> {
    // Generate a test image
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    let mut file = match File::create("image.ppm") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let line = format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    match file.write(line.as_bytes()) {
        Ok(_) => {}
        Err(error) => panic!("Problem writing the file: {:?}", error),
    };

    let mut j: i32 = IMAGE_HEIGHT - 1;

    while j >= 0 {
        print!("\rScan lines remaining: {}", j);
        std::io::stdout().flush().unwrap();

        let mut i = 0;
        while i < IMAGE_WIDTH {
            let r: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g: f64 = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            let line = format!("{ir} {ig} {ib}\n");

            match file.write(line.as_bytes()) {
                Ok(_) => {}
                Err(error) => panic!("Problem writing the file: {:?}", error),
            };
            i = i + 1;
        }
        j = j - 1;
    }
    println!("\nDone.\n");

    Ok(())
}
