use rand::rngs::SmallRng;
use rand::{RngExt, SeedableRng};
use std::sync::Arc;

mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod perlin;
mod quad;
mod ray;
mod rtw_image;
mod sphere;
mod texture;
mod vec3;

use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, MaterialPtr, Metal};
use crate::quad::Quad;
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture, Texture};
use crate::vec3::{Point3, Vec3};

fn bouncing_spheres() {
    let mut rng = SmallRng::from_rng(&mut rand::rng());

    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(checker)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.random::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rng.random::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.random::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo =
                        Color::random(&mut rng, 0.0, 1.0) * Color::random(&mut rng, 0.0, 1.0);
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, rng.random_range(0.0..0.5), 0.0);
                    world.add(Arc::new(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(&mut rng, 0.5, 1.0);
                    let fuzz = rng.random::<f64>() * 0.5;
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new_stationary(
                        center,
                        0.2,
                        sphere_material,
                    )));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new_stationary(
                        center,
                        0.2,
                        sphere_material,
                    )));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let world = BvhNode::from_list(world);

    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let samples_per_pixel: usize = 100;
    let max_depth: usize = 50;
    let background = Color::new(0.70, 0.80, 1.00);

    let vfov = 20.0;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );

    camera.render(&world);
}

fn checkered_spheres() {
    let mut world = HittableList::new();

    let checker: Arc<dyn Texture> = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::from_texture(Arc::clone(&checker))),
    )));

    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::from_texture(checker)),
    )));

    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let samples_per_pixel: usize = 100;
    let max_depth: usize = 50;
    let background = Color::new(0.70, 0.80, 1.00);

    let vfov = 20.0;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );

    cam.render(&world);
}

fn earth() {
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::from_texture(earth_texture));
    let globe = Sphere::new_stationary(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface);

    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let samples_per_pixel: usize = 100;
    let max_depth: usize = 50;
    let background = Color::new(0.70, 0.80, 1.00);

    let vfov = 20.0;
    let lookfrom = Point3::new(0.0, 0.0, 12.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );

    cam.render(&globe);
}

fn perlin_spheres() {
    let mut rng = SmallRng::from_rng(&mut rand::rng());

    let mut world = HittableList::new();

    let pertext: Arc<dyn Texture> = Arc::new(NoiseTexture::new(&mut rng, 4.0));

    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(Arc::clone(&pertext))),
    )));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::from_texture(pertext)),
    )));

    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let samples_per_pixel: usize = 100;
    let max_depth: usize = 50;
    let background = Color::new(0.70, 0.80, 1.00);

    let vfov = 20.0;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );

    cam.render(&world);
}

fn quads() {
    let mut world = HittableList::new();

    // Materials
    let left_red = Arc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    // Quads
    world.add(Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let aspect_ratio = 1.0;
    let image_width: usize = 400;
    let samples_per_pixel: usize = 100;
    let max_depth: usize = 50;
    let background = Color::new(0.70, 0.80, 1.00);

    let vfov = 80.0;
    let lookfrom = Point3::new(0.0, 0.0, 9.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );

    cam.render(&world);
}

fn simple_light() {
    let mut rng = SmallRng::from_rng(&mut rand::rng());

    let mut world = HittableList::new();

    let pertext: Arc<dyn Texture> = Arc::new(NoiseTexture::new(&mut rng, 4.0));

    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(Arc::clone(&pertext))),
    )));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::from_texture(pertext)),
    )));

    let difflight = Arc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Sphere::new_stationary(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight,
    )));

    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let samples_per_pixel: usize = 100;
    let max_depth: usize = 50;
    let background = Color::new(0.0, 0.0, 0.0);

    let vfov = 20.0;
    let lookfrom = Point3::new(26.0, 3.0, 6.0);
    let lookat = Point3::new(0.0, 2.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );

    cam.render(&world);
}

fn cornell_box() {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white,
    )));

    let aspect_ratio = 1.0;
    let image_width: usize = 600;
    let samples_per_pixel: usize = 200;
    let max_depth: usize = 50;
    let background = Color::new(0.0, 0.0, 0.0);

    let vfov = 40.0;
    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        background,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );

    cam.render(&world);
}

fn main() {
    match 7 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        _ => {}
    }
}
