mod camera;
mod hittable;
mod material;
mod ray;
mod utils;
mod vec3;

use std::convert::TryFrom;
use std::io;
use std::sync::Arc;
use std::thread;

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::utils::random_double;

struct Scene {
    camera: camera::Camera,
    world: HittableList,
    im_height: i32,
    im_width: i32,
    max_depth: i32,
}

fn ray_color(r: &ray::Ray, world: &HittableList, depth: i32) -> vec3::Color {
    let mut rec = hittable::HitRecord::new();
    // Too many bounces already
    if depth <= 0 {
        vec3::color(0., 0., 0.)
    } else {
        let (hit_any, idx) = world.hit(r, 0.001, f64::INFINITY, &mut rec);
        if hit_any {
            let (hit, attenuation, scattered) = world.objects[idx].get_material().scatter(r, &rec);
            if hit {
                ray_color(&scattered, world, depth - 1) * (attenuation)
            } else {
                vec3::color(0.0, 0.0, 0.0)
            }
        } else {
            let unit_direction: vec3::Vec3 = vec3::unit_vector(&r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            vec3::color(1.0, 1.0, 1.0) * (1. - t) + vec3::color(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {
    const N_THREADS: i32 = 4;
    // Image
    const RATIO: f64 = 16. / 9.;
    const IM_WIDTH: i32 = 1200;
    const MAX_DEPTH: i32 = 50;
    let sample_per_pixel = 500;
    let im_height: i32 = (f64::from(IM_WIDTH) / RATIO) as i32;

    // World
    let mut world = hittable::HittableList::new(vec![]);
    setup_world(&mut world);

    // Camera
    let lookfrom = vec3::point3(6., 2., 12.);
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
    //Scene
    let scene = Scene {
        camera,
        world,
        im_height,
        im_width: IM_WIDTH,
        max_depth: MAX_DEPTH,
    };
    let sc_arc = Arc::new(scene);
    let mut handles = vec![];
    let mut gr_vecs: Vec<Vec<vec3::Color>> = vec![];
    // Render
    print!("P3\n{} {}\n255\n", IM_WIDTH, im_height);

    for _ in 0..N_THREADS {
        let scene_thr_local = Arc::clone(&sc_arc);
        let handle = thread::spawn(move || {
            let gr_vec = compute_grid(
                &*scene_thr_local,
                im_height,
                IM_WIDTH,
                sample_per_pixel / N_THREADS,
            );
            gr_vec
        });
        handles.push(handle);
    }
    for handle in handles {
        match handle.join() {
            Ok(m) => gr_vecs.push(m),
            Err(e) => panic!(e),
        }
    }

    for j in (0..im_height).rev() {
        eprint!("\rScanning lines, remaining: {} ", j);
        for i in 0..IM_WIDTH {
            let idx = usize::try_from(j * IM_WIDTH + i).unwrap();
            let mut pixel_color = vec3::color(0., 0., 0.);
            for c in 0..N_THREADS {
                pixel_color += gr_vecs[usize::try_from(c).unwrap()][idx];
            }
            vec3::write_color(
                pixel_color,
                sample_per_pixel,
                io::stdout(),
            )
            .unwrap();
        }
    }
    eprintln!("Done")
}

fn compute_grid(scene: &Scene, im_height: i32, im_width: i32, samp: i32) -> Vec<vec3::Color> {
    let mut gr_vec: Vec<vec3::Color> =
        vec![vec3::color(0., 0., 0.); usize::try_from(im_height * im_width).unwrap()];
    for j in (0..im_height).rev() {
        eprint!("\rComputing lines, remaining: {} ", j);
        for i in 0..im_width {
            let idx = usize::try_from(j * im_width + i).unwrap();
            for _ in 0..samp {
                add_to_pixel(&mut gr_vec[idx], scene, i, j);
            }
        }
    }
    gr_vec
}

#[inline]
fn add_to_pixel(pixel_color: &mut vec3::Color, scene: &Scene, line: i32, col: i32) {
    let norm_lin = (f64::from(line) + random_double()) / f64::from(scene.im_width - 1);
    let norm_col = (f64::from(col) + random_double()) / f64::from(scene.im_height - 1);
    let ray: ray::Ray = scene.camera.get_ray(norm_lin, norm_col);
    *pixel_color += ray_color(&ray, &scene.world, scene.max_depth);
}

fn setup_world(world: &mut HittableList) {
    let ground_material = material::Lambertian::new(vec3::color(0.5, 0.5, 0.8));
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
                let material: Box<dyn material::Material + Send + Sync> = if choose_mat < 0.8 {
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
    let mat_left = material::Dielectric::new(1.5);
    world.objects.push(Box::new(hittable::Sphere::new(
        vec3::Point3::new(4., 1., 0.),
        1.0,
        Box::new(mat_left),
    )));
    let mat_left2 = material::Dielectric::new(1.5);
    world.objects.push(Box::new(hittable::Sphere::new(
        vec3::Point3::new(4., 1., 0.),
        -0.8,
        Box::new(mat_left2),
    )));
    let mat_right = material::Metal::new(vec3::color(0.7, 0.6, 0.5), 0.0);
    world.objects.push(Box::new(hittable::Sphere::new(
        vec3::Point3::new(-4., 1., 0.),
        1.0,
        Box::new(mat_right),
    )));
    let mat_more = material::Dielectric::new(1.5);
    let sphere_more = Box::new(hittable::Sphere::new(
        vec3::Point3::new(0., 1., -5.),
        3.0,
        Box::new(mat_more) as Box<dyn material::Material + Send + Sync>,
    )) as Box<dyn hittable::Hittable + Send + Sync>;
    world.objects.push(sphere_more);
}
