use std::{fs::File, io::BufReader};

use crate::{math::Vec3, spectrum::RGB};

pub struct Texture {
    pub pixel_data: Vec<RGB>,
    pub width: usize,
    pub height: usize,
}

impl Texture {
    pub fn set_earth() -> Self {
        // https://www.solarsystemscope.com/textures/
        let file = File::open("assets/2k_earth_daymap.jpg");
        if let Err(_) = file {
            return Texture {
                pixel_data: vec![Vec3::new(0.1)],
                width: 1,
                height: 1,
            };
        }

        let file = file.unwrap();
        let mut decoder = jpeg_decoder::Decoder::new(BufReader::new(file));
        let pixels = decoder.decode().expect("failed to decode image");
        let metadata = decoder.info().unwrap();

        let width = metadata.width as usize;
        let height = metadata.height as usize;
        let mut data = vec![Vec3::zero(); width * height];

        for i in 0..width * height {
            let r = pixels[3 * i] as f64 / 255.;
            let g = pixels[3 * i + 1] as f64 / 255.;
            let b = pixels[3 * i + 2] as f64 / 255.;
            data[i] = Vec3(r.powf(2.2), g.powf(2.2), b.powf(2.2));
        }

        Texture {
            pixel_data: data,
            width,
            height,
        }
    }

    pub fn get_rgb(&self, u: f64, v: f64) -> RGB {
        let w = (self.width as f64 * u) as usize;
        let h = (self.height as f64 * v) as usize;
        self.pixel_data[h * self.width + w]
    }
}
