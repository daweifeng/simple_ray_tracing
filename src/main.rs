use std::fs::File;
use std::io::Write;

mod color;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use ray::Ray;
use vec3::Vec3 as Point; // 3D point
use vec3::Vec3;

fn main() -> std::io::Result<()> {
    // Image
    const ASPECT: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT) as i32;

    // Camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin: Point = Point(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical: Vec3 = Vec3(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, FOCAL_LENGTH);

    let mut file = match File::create("image.ppm") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // Renderer

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
            let u: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v: f64 = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r: Ray = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };

            let pixel_color = r.ray_color();
            color::write_color(&mut file, pixel_color);
            i = i + 1;
        }
        j = j - 1;
    }
    println!("\nDone.\n");

    Ok(())
}
