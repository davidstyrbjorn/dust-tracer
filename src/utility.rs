pub use crate::prelude::*;

pub fn to_png_color(color: &Vector3<f64>) -> (u8, u8, u8) {
    let r = (255.999 * color.x) as u8;
    let g = (255.999 * color.y) as u8;
    let b = (255.999 * color.z) as u8;
    (r, g, b)
}

pub fn hit_sphere(center: Vector3<f64>, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.orig - center;
    let a = ray.dir.dot(&ray.dir);
    let b = 2.0 * oc.dot(&ray.dir);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}

pub fn ray_color(r: &Ray) -> Vector3<f64> {
    let t = hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.2, r);
    if t > 0.0 {
        // Normal pointing outward
        let normal = (r.at(t) - Vector3::new(0.0, 0.0, -1.0)).normalize();
        return 0.5 * Vector3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0);
    }
    let norm = r.dir / r.dir.magnitude();
    let t = (0.5 * norm.y) + 1.0;
    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}
