extern crate image;
extern crate nalgebra as na;
extern crate rand;
use std::fs::File;
use std::path::Path;
use rand::distributions::{IndependentSample, Range};
use na::{Vector3, Norm, Dot};

type Vec3 = Vector3<f32>; 


fn main() {

	let nx = 200;
	let ny = 100;
	let ns = 100;
	// Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    let cam = Camera::new();
    let range = Range::new(0.0, 1.0);
    let mut rng = rand::thread_rng();

	// set up scene
	let worldvec: Vec<Box<Hitable>> = vec![
		Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian::new(0.8, 0.3, 0.3)))),
		Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian::new(0.8, 0.8, 0.0)))),
		Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(0.8, 0.6, 0.2, 0.0)))),
		Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(0.8, 0.8, 0.8, 0.3))))
	];
	let world = HitableList::new(worldvec);

    // Iterate over the coordiantes and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    	let mut col = Vec3::new(0.0, 0.0, 0.0);
    	for _ in 0..ns {
	    	let u = (x as f32 + range.ind_sample(&mut rng)) / nx as f32 ;
	    	let v = ((ny - y) as f32 + range.ind_sample(&mut rng)) / ny as f32;
	    	let r = cam.get_ray(u, v);
	    	// let p = r.point_at_t(2.0);
	    	col = col + color(&r, &world, 0);
	    }
	    col = col / (ns as f32);
	    col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
        *pixel = image::Rgb([(col.x*255.99) as u8, (col.y*255.99) as u8, (col.z*255.99) as u8]);
    }
    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("output.png")).unwrap();
    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);

    println!("Finished Render");
}

fn color(ray: &Ray, world: &Hitable, depth: i32) -> Vec3 {
	let rec = world.hit(ray, 0.001, std::f32::MAX);
	if rec.hit {
		if depth < 50 {
			match rec.material {
			    Some(mat) => {
			    	let (cont, attenuation, scattered) = mat.scatter(ray, &rec);
			    	if cont {
						attenuation*color(&scattered, world, depth+1)
					} else {
						Vec3::new(0.0, 0.0, 0.0)
					}
			    },
			    None => Vec3::new(0.0, 0.0, 0.0),
			}
		}
		else {
			Vec3::new(0.0, 0.0, 0.0)
		}
	}
	else {
		let t = 0.5 * (ray.direction.normalize().y + 1.0);
		(1.0 - t) * Vec3::new(1.0, 1.0 , 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
	}
}

struct Ray {
	origin: Vec3,
	direction: Vec3,
}

impl Ray {
	fn new(origin: Vec3, direction: Vec3) -> Ray {
		Ray {
			origin: origin,
			direction: direction,
		}
	}

	fn point_at_t(&self, t: f32) -> Vec3 {
		self.origin + (self.direction * t)
	}
}

struct HitRecord<'a> {
	material: Option<&'a Box<Material>>,
	p: Vec3,
	normal: Vec3,
	t: f32,
	hit: bool,
}

impl<'a> HitRecord<'a> {
	fn new() -> HitRecord<'a> {
		HitRecord {
			material: None,
			p: Vec3::new(0.0, 0.0, 0.0),
			normal: Vec3::new(0.0, 0.0, 0.0),
			t: 0.0,
			hit: false,
		}
	}
}

trait Hitable {
	fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> HitRecord;
}

struct Sphere {
	material: Box<Material>,
	center: Vec3,
	radius: f32,
}

impl Sphere {
	fn new(center: Vec3, radius: f32, material: Box<Material>) -> Sphere {
		Sphere {
			material: material,
			center: center,
			radius: radius,
		}
	}
}

fn random_in_unit_sphere() -> Vec3 {
	let range = Range::new(-1.0, 1.0);
    let mut rng = rand::thread_rng();
	loop {
		let p = 2.0* Vec3::new(range.ind_sample(&mut rng), range.ind_sample(&mut rng), range.ind_sample(&mut rng));
		if p.norm_squared() > 1.0 {
			return p
		}
	}
}

impl Hitable for Sphere {
	fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> HitRecord {
		let oc = ray.origin - self.center;
		let a = ray.direction.norm_squared();
		let b = ray.direction.dot(&oc);
		let c = oc.norm_squared() - self.radius*self.radius;
		let d = b*b - a*c; // discriminant
		if d > 0.0 {
			let t1 = (-b) / a;
			let t2 = (b*b - a*c).sqrt() / a;
			let temp1 = t1 - t2;
			let temp2 = t1 + t2;
			if temp1 < t_max && temp1 > t_min {
				let p = ray.point_at_t(temp1);
				HitRecord {
					material: Some(&self.material),
					p: p,
					normal: (p - self.center) / self.radius,
					t: temp1,
					hit: true,
				}
			} 
			else if temp2 < t_max && temp2 > t_min {
				let p = ray.point_at_t(temp2);
				HitRecord {
					material: Some(&self.material),
					p: p,
					normal: (p - self.center) / self.radius,
					t: temp2,
					hit: true,
				}
			}
			else {
				HitRecord::new()

			}
		}
		else {
			HitRecord::new()
		}
	}
}

struct HitableList {
	list: Vec<Box<Hitable>>,
}

impl HitableList {
	fn new(list: Vec<Box<Hitable>>) -> HitableList {
		HitableList {
			list: list,
		}
	}
}


impl Hitable for HitableList {
	fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> HitRecord {
		let mut closest = t_max;
		let mut record = HitRecord::new();
		for item in self.list.iter() {
			let temp = item.hit(ray, t_min, closest);
			if temp.hit {
				closest = temp.t;
				record = temp;
			}
		}
		record
	}
}

struct Camera {
	origin: Vec3,
	llc: Vec3, // lower left corner
	horizont: Vec3,
	vertical: Vec3,
}

impl Camera {
	fn new() -> Camera {
		Camera {
			origin: Vec3::new(0.0, 0.0, 0.0),
			llc: Vec3::new(-2.0, -1.0, -1.0),
			horizont: Vec3::new(4.0, 0.0, 0.0),
			vertical: Vec3::new(0.0, 2.0, 0.0),
		}
	}

	fn get_ray(&self, u: f32, v: f32) -> Ray{
		Ray::new(self.origin, self.llc + u*self.horizont + v*self.vertical - self.origin)
	}
}

trait Material {
	fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray);
}

struct Lambertian {
	albedo: Vec3,
}

impl Lambertian {
	fn new(r: f32, g: f32, b: f32) -> Lambertian {
		Lambertian {albedo: Vec3::new(r, g, b)}
	}
}

impl Material for Lambertian {
	fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray) {
		let target = rec.p + rec.normal + random_in_unit_sphere();
		let scattered = Ray::new(rec.p, target-rec.p);
		(true, self.albedo, scattered)
	}
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
	*v - ((v.dot(n) * 2.0) * *n)
}

struct Metal {
	albedo: Vec3,
	fuzz: f32,
}

impl Metal {
	fn new(r: f32, g: f32, b: f32, fuzz: f32) -> Metal {
		 
		Metal {
			albedo: Vec3::new(r, g, b),
			fuzz: if fuzz < 0.0 || fuzz > 1.0 { 1.0 } else { fuzz },
		}
	}
}

impl Material for Metal {
	fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray) {
		let reflected = reflect(&ray.direction.normalize(), &rec.normal);
		let scattered = Ray::new(rec.p, reflected + self.fuzz*random_in_unit_sphere());
		(scattered.direction.dot(&rec.normal) > 0.0, self.albedo, scattered)
	}
}