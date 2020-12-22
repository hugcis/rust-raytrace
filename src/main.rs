mod hittable;
mod ray;
mod vec3;
mod camera;
mod utils;
mod material;

use std::io;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::utils::random_double;


fn ray_color(r: &ray::Ray, world: &impl Hittable, depth: i32) -> vec3::Color {
    let mut rec = hittable::HitRecord::new();
    if depth <= 0 {
        vec3::color(0., 0., 0.)
    }
    else if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let (hit, attenuation, scattered) = rec.material.scatter(r, &rec);
        ray_color(scattered, world, depth - 1) * (*attenuation)
    } else {
        let unit_direction: vec3::Vec3 = vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        vec3::color(1.0, 1.0, 1.0) * (1. - t) + vec3::color(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    // Image
    const RATIO: f64 = 16. / 9.;
    const IM_WIDTH: i16 = 400;
    const MAX_DEPTH: i32 = 50;
    let sample_per_pixel = 100;
    let im_height: i16 = (f64::from(IM_WIDTH) / RATIO) as i16;
    // World
    let mut world = hittable::HittableList { objects: vec![] };
    world.objects.push(Box::new(hittable::Sphere::new(
        vec3::Point3::new(0., 0., -1.),
        0.5,
    )));
    world.objects.push(Box::new(hittable::Sphere::new(
        vec3::Point3::new(0., -100.5, -1.),
        100.,
    )));
    // Camera
    let camera = Camera::new();
    // Render
    print!("P3\n{} {}\n255\n", IM_WIDTH, im_height);
    for j in (0..im_height).rev() {
        eprint!("\rScanning lines, remaining: {} ", j);
        for i in 0..IM_WIDTH {
            let mut pixel_color = vec3::color(0., 0., 0.);
            for _ in 0..sample_per_pixel {
                let u = (f64::from(i) + random_double()) / f64::from(IM_WIDTH - 1);
                let v = (f64::from(j) + random_double()) / f64::from(im_height - 1);
                let r: ray::Ray = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            vec3::write_color(pixel_color, sample_per_pixel, io::stdout()).unwrap();
        }
    }
    eprintln!("Done")
}
