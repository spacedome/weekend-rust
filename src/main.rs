extern crate image;
extern crate nalgebra as na;
use na::{Vector3, Norm, Dot};

type Vec3 = Vector3<f32>; 

use std::fs::File;
use std::path::Path;


fn main() {

	let imgx = 200;
	let imgy = 100;

	// Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    let llc = Vec3::new(-2.0, -1.0, -1.0);
	let horizont = Vec3::new(4.0, 0.0, 0.0);
	let vertical = Vec3::new(0.0, 2.0, 0.0);
	let origin = Vec3::new(0.0, 0.0, 0.0);

    // Iterate over the coordiantes and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    	let u = x as f32 / imgx as f32;
    	let v = (imgy - y) as f32 / imgy as f32;
    	let r = Ray::new(origin, llc + (u*horizont) + (v*vertical));
    	let col = color(&r);
        *pixel = image::Rgb([(col.x*255.99) as u8, (col.y*255.99) as u8, (col.z*255.99) as u8]);
    }
    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("output.png")).unwrap();
    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);

    println!("Finished Render");
}


struct Ray {
	origin: Vector3<f32>,
	direction: Vector3<f32>,
}

impl Ray {
	fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
		Ray {
			origin: origin,
			direction: direction,
		}
	}

	fn point_at_t(&self, t: f32) -> Vector3<f32> {
		self.origin + (self.direction * t)
	}
}


fn color(ray: &Ray) -> Vector3<f32> {
	if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray) { return Vec3::new(1.0, 0.0, 0.0) }
	let unit_dir = ray.direction.normalize();
	let t: f32 = 0.5*(unit_dir.y + 1.0);
	Vector3::new(1.0-t, 1.0-t, 1.0-t) + (t*Vector3::new(0.5, 0.7, 1.0))
}

fn hit_sphere(center: &Vector3<f32>, radius: f32, r: &Ray) -> bool {
	let oc = r.origin - *center;
	let a = r.direction.norm_squared();
	let b = 2.0 * r.direction.dot(&oc);
	let c = oc.norm_squared() - radius*radius;
	b*b - 4.0*a*c > 0.0
}