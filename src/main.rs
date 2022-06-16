mod aabb;
mod bvh;
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

use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList};
use crate::utils::random_double;

struct Scene {
    camera: camera::Camera,
    world: BVHNode,
    im_height: i32,
    im_width: i32,
    max_depth: i32,
}

fn ray_color(r: &ray::Ray, world: &BVHNode, depth: i32) -> vec3::Color {
    // Too many bounces already
    if depth <= 0 {
        vec3::color(0., 0., 0.)
    } else {
        let hit_any = world.hit(r, 0.001, f64::INFINITY);
        match hit_any {
            Some(rec) => rec
                .mat
                .as_ref()
                .expect("Hit recorded with no material.")
                .scatter(r, &rec)
                .map_or(vec3::color(0.0, 0.0, 0.0), |(attenuation, scattered)| {
                    ray_color(&scattered, world, depth - 1) * (attenuation)
                }),
            None => {
                let unit_direction: vec3::Vec3 = vec3::unit_vector(&r.direction());
                let t = 0.5 * (unit_direction.y() + 1.0);
                vec3::color(1.0, 1.0, 1.0) * (1. - t) + vec3::color(0.5, 0.7, 1.0) * t
            }
        }
    }
}

fn ray_color(r: &mut ray::Ray, world: &BVHNode, depth: i32) -> vec3::Color {
    // Too many bounces already
    let radiance = vec3::color(0., 0., 0.);
    for i in 0..depth {
        let hit_any = world.hit(r, 0.001, f64::INFINITY);
        radiance += match hit_any {
            Some(rec) => {
                match rec
                    .mat
                    .as_ref()
                    .expect("Hit recorded with no material.")
                    .scatter(r, &rec) {
                        Some((attenuation, scattered_ray)) => {

                        },
                        None => {
                            *ray
                        }
                    }

            }
            None => {
                let unit_direction: vec3::Vec3 = vec3::unit_vector(&r.direction());
                let t = 0.5 * (unit_direction.y() + 1.0);
                vec3::color(1.0, 1.0, 1.0) * (1. - t) + vec3::color(0.5, 0.7, 1.0) * t
            }
        };
    }
    radiance
}

fn main() {
    const N_THREADS: i32 = 8;
    // Image
    const RATIO: f64 = 16. / 9.;
    const IM_WIDTH: i32 = 1200;
    const MAX_DEPTH: i32 = 100;
    let sample_per_pixel = 500;
    let im_height: i32 = (f64::from(IM_WIDTH) / RATIO) as i32;

    // World
    let mut world = hittable::HittableList::new(vec![]);
    setup_world(&mut world);
    let hit_list: Vec<Box<dyn Hittable + Send + Sync>> = world.objects;
    let bvh = BVHNode::new(hit_list, 0., 0.);

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
        world: bvh,
        im_height,
        im_width: IM_WIDTH,
        max_depth: MAX_DEPTH,
    };
    let sc_arc = Arc::new(scene);
    let mut handles = vec![];
    // let mut gr_vecs: Vec<Vec<vec3::Color>> = vec![];
    // Render
    print!("P3\n{} {}\n255\n", IM_WIDTH, im_height);

    for _ in 0..N_THREADS {
        let scene_thr_local = Arc::clone(&sc_arc);
        let handle = thread::spawn(move || {
            compute_grid(
                &*scene_thr_local,
                im_height,
                IM_WIDTH,
                sample_per_pixel / N_THREADS,
            )
        });
        handles.push(handle);
    }
    let gr_vecs: Vec<Vec<vec3::Color>> = handles.into_iter().flat_map(|h| h.join()).collect();

    for j in (0..im_height).rev() {
        eprint!("\rScanning lines, remaining: {} ", j);
        for i in 0..IM_WIDTH {
            let idx = usize::try_from(j * IM_WIDTH + i).unwrap();
            let mut pixel_color = vec3::color(0., 0., 0.);
            for c in 0..N_THREADS {
                pixel_color += gr_vecs[usize::try_from(c).unwrap()][idx];
            }
            vec3::write_color(pixel_color, sample_per_pixel, io::stdout()).unwrap();
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

fn make_random_sphere(world: &mut HittableList, center: vec3::Point3, radius: f64) {
    let choose_mat = utils::random_double();
    if (center - vec3::point3(4., radius, 0.)).length() > 0.9 {
        if choose_mat < 0.6 {
            let color = vec3::Color::random() * vec3::Color::random();
            world.objects.push(Box::new(hittable::Sphere::new(
                center,
                radius,
                material::Lambertian::new(color),
            )));
        } else if choose_mat < 0.95 {
            let color = vec3::Color::random_range(0.5, 1.);
            let fuzz = utils::random_double_range(0., 0.5);
            world.objects.push(Box::new(hittable::Sphere::new(
                center,
                radius,
                material::Metal::new(color, fuzz),
            )));
        } else {
            world.objects.push(Box::new(hittable::Sphere::new(
                center,
                radius,
                material::Dielectric::new(1.5),
            )));
        };
    }
}

fn setup_world(world: &mut HittableList) {
    let ground_material = material::Lambertian::new(vec3::color(0.5, 0.5, 0.8));
    world.objects.push(Box::new(hittable::Sphere::new(
        vec3::Point3::new(0., -1000., 0.),
        1000.,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let posx = f64::from(a) + 0.9 * random_double();
            let posy = 0.2;
            let posz = f64::from(b) + 0.9 * random_double();
            let center = vec3::point3(posx, posy, posz);
            make_random_sphere(world, center, 0.2);
        }
    }
    for a in -22..22 {
        for b in -22..22 {
            let posx = f64::from(a) / 2. + 0.9 * random_double();
            let posy = 0.05;
            let posz = f64::from(b) / 2. + 0.9 * random_double();
            let center = vec3::point3(posx, posy, posz);
            make_random_sphere(world, center, 0.05);
        }
    }
    for a in -33..33 {
        for b in -33..33 {
            let posx = f64::from(a) / 3. + 0.9 * random_double();
            let posy = 0.02;
            let posz = f64::from(b) / 3. + 0.9 * random_double();
            let center = vec3::point3(posx, posy, posz);
            make_random_sphere(world, center, 0.02);
        }
    }

    let mat_center = material::Dielectric::new(1.5);
    world.objects.push(Box::new(hittable::Sphere::new(
        vec3::Point3::new(0., 1., 0.),
        1.0,
        mat_center,
    )));
    let mat_left = material::Dielectric::new(1.5);
    world.objects.push(Box::new(hittable::Sphere::new(
        vec3::Point3::new(4., 1., 0.),
        1.0,
        mat_left,
    )));
    let mat_left2 = material::Dielectric::new(1.5);
    world.objects.push(Box::new(hittable::Sphere::new(
        vec3::Point3::new(4., 1., 0.),
        -0.8,
        mat_left2,
    )));
    let mat_right = material::Metal::new(vec3::color(0.7, 0.6, 0.5), 0.0);
    world.objects.push(Box::new(hittable::Sphere::new(
        vec3::Point3::new(-4., 1., 0.),
        1.0,
        mat_right,
    )));
    let mat_more = material::Dielectric::new(1.5);
    let sphere_more = Box::new(hittable::Sphere::new(
        vec3::Point3::new(0., 1.5, -5.),
        1.5,
        mat_more,
    ));
    world.objects.push(sphere_more);
}
