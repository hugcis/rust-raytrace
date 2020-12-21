mod vec3;
mod ray;

use std::io;

fn hit_sphere(center: vec3::Point3, radius: f64, r: ray::Ray) -> f64 {
    let oc = *r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = vec3::dot(oc, *r.direction());
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0. {
        -1.0
    } else {
        (-half_b - discriminant.sqrt() ) / a
    }
}

fn ray_color(r: &ray::Ray) -> vec3::Color {
    let sp = hit_sphere(vec3::Point3::new(0., 0., -1.), 0.5,  *r);
    if sp > 0.0 {
        let normal = vec3::unit_vector(&(r.at(sp) - vec3::Vec3::new(0., 0., -1.)));
        vec3::color(normal.x() + 1., normal.y() + 1., normal.z() + 1.) * 0.5
    }
    else {
        let unit_direction: vec3::Vec3 = vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        vec3::color(1.0, 1.0, 1.0) * (1_f64 - t) + vec3::color(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    // Image
    const RATIO: f64 = 16./9.;
    const IM_WIDTH: i16 = 800;
    let im_height: i16 = (f64::from(IM_WIDTH) / RATIO) as i16;
    // Camera

    let viewport_height = 2.0;
    let viewport_width = RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Point3::new(0., 0., 0.);
    let horizontal = vec3::Vec3::new(viewport_width, 0., 0.);
    let vertical = vec3::Vec3::new(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal/2. - vertical/2. -
        vec3::Vec3::new(0., 0., focal_length);
    // Render
    print!("P3\n{} {}\n255\n", IM_WIDTH, im_height);
    for j in (0..im_height).rev() {
        eprint!("\rScanning lines, remaining: {} ", j);
        for i in 0..IM_WIDTH {
            let u = f64::from(i)/f64::from(IM_WIDTH - 1);
            let v = f64::from(j)/f64::from(im_height - 1);
            let r: ray::Ray = ray::Ray::new(origin,
                                            lower_left_corner +
                                            horizontal * u +
                                            vertical * v - origin);
            let pixel_color: vec3::Color = ray_color(&r);
            vec3::write_color(pixel_color, io::stdout()).unwrap();
        }
    }
    eprintln!("Done")
}
