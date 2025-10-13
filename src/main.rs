use crate::{
    camera::{Camera, Direction, Eye},
    render::render,
    scene::Scene,
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

fn main() {
    let scene = Scene::new(3);

    let eye = Eye::new(18, 30., 1., Direction::West, 30.);
    let camera = Camera::new(&eye, &scene, 600, 400, 0.2, 0.5, 4, 4);
    render(&scene, &camera);
}
