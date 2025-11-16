use crate::{
    camera::{Camera, Direction, Eye},
    render::render,
    scene::Scene,
    texture::Texture,
};

mod camera;
mod constant;
mod math;
mod pathtracing;
mod random;
mod ray;
mod render;
mod sampling;
mod scene;
mod spectrum;
mod sphere;
mod texture;

#[allow(unused)]
fn render_bluesky() {
    let texture = Texture::new();
    let mut scene = Scene::new(3, &texture);
    let eye = Eye::new(12., 30., 1., Direction::South, 30.);
    let camera = Camera::new(&eye, &mut scene, 600, 400, 0.2, 0.5, 4, 4);
    render(&scene, &camera);
}

#[allow(unused)]
fn render_sunset() {
    let texture = Texture::new();
    let mut scene = Scene::new(3, &texture);
    let eye = Eye::new(18., 30., 1., Direction::West, 30.);
    let camera = Camera::new(&eye, &mut scene, 600, 400, 0.2, 0.5, 4, 4);
    render(&scene, &camera);
}

#[allow(unused)]
fn render_earth() {
    let texture = Texture::new();
    let mut scene = Scene::new(3, &texture);
    let eye = Eye::new(12., 35., 6000., Direction::South, -90.);
    let camera = Camera::new(&eye, &mut scene, 600, 400, 0.2, 0.5, 4, 4);
    render(&scene, &camera);
}

fn main() {
    let start = std::time::Instant::now();

    //render_bluesky();
    //render_sunset();
    render_earth();

    let end = start.elapsed();
    println!("{}.{:03}sec", end.as_secs(), end.subsec_nanos() / 1000000);
}
