use bmp::{Image, Pixel, px};
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

use crate::{
    camera::Camera,
    math::{Vec3, is_valid},
    pathtracing::Pathtracing,
    random::XorRand,
    ray::Ray,
    sampling::{pdf_sample_wavelength, sample_wavelength},
    scene::Scene,
    spectrum::{color_matching, convert_to_srgb, gamma},
};

pub fn render(scene: &Scene, camera: &Camera) {
    let total_px = camera.pixel_num_w * camera.pixel_num_h;
    let mut buffer = vec![Vec3::zero(); total_px as usize];
    let mut img = Image::new(camera.pixel_num_w, camera.pixel_num_h);
    let coeff_inv =
        1. / ((camera.spp * camera.sspp.pow(2)) as f64 * 106.91973463815504343620398716858);

    buffer
        .par_chunks_mut(camera.pixel_num_w as usize)
        .enumerate()
        .for_each(|(v, row)| {
            for u in 0..camera.pixel_num_w {
                let mut rand = XorRand::new(u * v as u32);
                let mut accumlated_value = Vec3::zero();
                for sv in 0..camera.sspp {
                    for su in 0..camera.sspp {
                        let pos_on_sensor =
                            camera.get_pixel_center(u as f64, v as f64, su as f64, sv as f64);
                        let dir = (pos_on_sensor - camera.eye_pos).normalize();
                        for _ in 0..camera.spp {
                            let wavelength = sample_wavelength(&mut rand);
                            let col_matching = color_matching(wavelength);
                            let mut tracer =
                                Pathtracing::new(Ray::new(pos_on_sensor, dir), wavelength);

                            let value = tracer.integrate(scene, &mut rand);
                            if is_valid(value) {
                                accumlated_value = accumlated_value
                                    + value * col_matching / pdf_sample_wavelength(wavelength);
                            }
                        }
                    }
                }
                row[u as usize] = accumlated_value * coeff_inv;
            }
            println!("{v}");
        });

    for i in 0..total_px {
        let v = i / camera.pixel_num_w;
        let u = i - camera.pixel_num_w * v;
        let xyz = buffer[i as usize];

        let rgb = gamma(convert_to_srgb(&xyz));
        img.set_pixel(u, v as u32, px!(rgb.0, rgb.1, rgb.2));
    }
    let _ = img.save("reference.bmp");
}
