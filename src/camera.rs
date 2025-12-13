use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::io::{self, Write};

use crate::color;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    aspect_ratio: f64,
    image_width: usize,  // Rendered image width in pixel count
    image_height: usize, // Rendered image height

    vfov: f64,        // Vertical view angle (field of view)
    lookfrom: Point3, //Point camera is looking from
    lookat: Point3,   // Point camera is looking at
    vup: Vec3,        // Camera-relative "up" direction

    defocus_angle: f64, // Variation angle of rays through each pixel
    focus_dist: f64,    // Distance from camera lookfrom point to plane of perfect focus

    samples_per_pixel: usize, // Count of random samples for each pixel
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    max_depth: usize,         // Maximum number of ray bounces into scene

    center: Point3,      // Camera center
    pixel00_loc: Point3, // Location of pixle 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below

    // Camera frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,

    defocus_disk_u: Vec3, // Defocus disk horizontal radius
    defocus_disk_v: Vec3, // Defocus disk vertical radius
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: usize,
        samples_per_pixel: usize,
        max_depth: usize,
        vfov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let calculated_height = ((image_width as f64) / aspect_ratio) as usize;
        let image_height = calculated_height.max(1);

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let center = lookfrom;

        // Determine viewport dimensions.
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectorrs for the camera coordinate frame.
        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -v; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            aspect_ratio,
            image_width,
            image_height,

            vfov,
            lookfrom,
            lookat,
            vup,

            defocus_angle,
            focus_dist,

            samples_per_pixel,
            pixel_samples_scale,
            max_depth,

            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,

            u,
            v,
            w,

            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        let mut rng = SmallRng::from_rng(&mut rand::rng());

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            io::stderr().flush().unwrap();
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j, &mut rng);
                    pixel_color += Self::ray_color(&r, self.max_depth, world, &mut rng);
                }
                color::write_color(&(self.pixel_samples_scale * pixel_color));
            }
        }
        eprintln!("\nDone.                 ");
    }

    fn ray_color(r: &Ray, depth: usize, world: &dyn Hittable, rng: &mut SmallRng) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::zero();
        }

        if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec, rng) {
                return attenuation * Self::ray_color(&scattered, depth - 1, world, rng);
            } else {
                return Color::zero();
            }
        }

        let unit_direction = r.dir.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, i: usize, j: usize, rng: &mut SmallRng) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.

        let offset = self.sample_square(rng);
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample(rng)
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = rng.random_range(0.0..1.0);

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn sample_square(&self, rng: &mut SmallRng) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(
            rng.random_range(-0.5..0.5),
            rng.random_range(-0.5..0.5),
            0.0,
        )
    }

    fn defocus_disk_sample(&self, rng: &mut SmallRng) -> Point3 {
        // Returns a random point in the camera defocus disk.
        let p = Vec3::random_in_unit_disk(rng);
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}
