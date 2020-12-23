mod camera;
mod hittable;
mod material;
mod ray;
mod utils;
mod vec3;

use std::io;

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::utils::random_double;

fn ray_color(r: &ray::Ray, world: &HittableList, depth: i32) -> vec3::Color {
    let mut rec = hittable::HitRecord::new();
    if depth <= 0 {
        vec3::color(0., 0., 0.)
    } else {
        let (hit_any, idx) = world.hit(r, 0.001, f64::INFINITY, &mut rec);
        match hit_any {
            true => {
                let (hit, attenuation, scattered) =
                    world.objects[idx].get_material().scatter(r, &rec);
                match hit {
                    true => ray_color(&scattered, world, depth - 1) * (attenuation),
                    false => vec3::color(0.0, 0.0, 0.0),
                }
            }
            false => {
                let unit_direction: vec3::Vec3 = vec3::unit_vector(&r.direction());
                let t = 0.5 * (unit_direction.y() + 1.0);
                vec3::color(1.0, 1.0, 1.0) * (1. - t) + vec3::color(0.5, 0.7, 1.0) * t
            }
        }
    }
}

fn main() {
    // Image
    const RATIO: f64 = 3. / 2.;
    const IM_WIDTH: i16 = 1200;
    const MAX_DEPTH: i32 = 50;
    let sample_per_pixel = 500;
    let im_height: i16 = (f64::from(IM_WIDTH) / RATIO) as i16;
    // World
    let mut world = hittable::HittableList::new(vec![]);

    let ground_material = material::Lambertian::new(vec3::color(0.5, 0.5, 0.5));
    world.objects.push(Box::new(hittable::Sphere::new(
        vec3::Point3::new(0., -1000., 0.),
        1000.,
        Box::new(ground_material),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utils::random_double();
            let center = vec3::point3(
                f64::from(a) + 0.9 * random_double(),
                0.2,
                f64::from(b) + 0.9 * random_double(),
            );
            if (center - vec3::point3(4., 0.2, 0.)).length() > 0.9 {
                let material: Box<dyn material::Material> = if choose_mat < 0.8 {
                    let color = vec3::Color::random() * vec3::Color::random();
                    Box::new(material::Lambertian::new(color))
                } else if choose_mat < 0.95 {
                    let color = vec3::Color::random_range(0.5, 1.);
                    let fuzz = utils::random_double_range(0., 0.5);
                    Box::new(material::Metal::new(color, fuzz))
                } else {
                    Box::new(material::Dielectric::new(1.5))
                };
                world
                    .objects
                    .push(Box::new(hittable::Sphere::new(center, 0.2, material)));
            }
        }
    }

    let mat_center = material::Dielectric::new(1.5);
    world.objects.push(Box::new(hittable::Sphere::new(
        vec3::Point3::new(0., 1., 0.),
        1.0,
        Box::new(mat_center),
    )));
    let mat_left = material::Lambertian::new(vec3::color(0.4, 0.2, 0.1));
    world.objects.push(Box::new(hittable::Sphere::new(
        vec3::Point3::new(-4., 1., 0.),
        1.0,
        Box::new(mat_left),
    )));
    let mat_right = material::Metal::new(vec3::color(0.7, 0.6, 0.5), 0.0);
    world.objects.push(Box::new(hittable::Sphere::new(
        vec3::Point3::new(4., 1., 0.),
        1.0,
        Box::new(mat_right),
    )));

    // Camera
    let lookfrom = vec3::point3(13., 2., 3.);
    let lookat = vec3::point3(0., 0., 0.);
    let aperture = 0.1;
    let dist_to_focus = 10.;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vec3::Vec3::new(0., 1., 0.),
        20.0,
        RATIO,
        aperture,
        dist_to_focus,
    );
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
