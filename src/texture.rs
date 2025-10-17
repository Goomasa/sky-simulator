use std::{fs::File, io::BufReader};

use tiff::decoder::{Decoder, DecodingResult};
use zune_jpeg::JpegDecoder;

use crate::{math::Vec3, spectrum::RGB};

pub struct Texture {
    pub rgb_data: Vec<RGB>,
    pub specular_data: Option<Vec<u8>>,
    pub normal_data: Option<Vec<Vec3>>,
    pub width: usize,
    pub height: usize,
}

impl Texture {
    pub fn new() -> Self {
        let (rgb_data, width, height) = load_jpg();
        let (specular_data, normal_data) = load_tiff(width, height);
        Texture {
            rgb_data,
            specular_data,
            normal_data,
            width,
            height,
        }
    }

    pub fn get_rgb(&self, u: f64, v: f64) -> RGB {
        let w = (self.width as f64 * u) as usize;
        let h = (self.height as f64 * v) as usize;
        self.rgb_data[h * self.width + w]
    }

    pub fn get_property(&self, u: f64, v: f64) -> (u8, Vec3) {
        let w = (self.width as f64 * u) as usize;
        let h = (self.height as f64 * v) as usize;
        let id = h * self.width + w;

        let specular = if let Some(sdata) = &self.specular_data {
            sdata[id]
        } else {
            0
        };

        let normal = if let Some(ndata) = &self.normal_data {
            ndata[id]
        } else {
            Vec3(0., 0., 1.)
        };

        (specular, normal)
    }
}

fn load_jpg() -> (Vec<RGB>, usize, usize) {
    // https://www.solarsystemscope.com/textures/
    let file = File::open("assets/2k_earth_daymap.jpg");
    if let Err(_) = file {
        println!("rgb-texture not found");
        return (vec![Vec3::new(0.1)], 1, 1);
    }

    let file = BufReader::new(file.unwrap());
    let mut decoder = JpegDecoder::new(file);
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

    (data, width, height)
}

fn load_tiff(w: usize, h: usize) -> (Option<Vec<u8>>, Option<Vec<RGB>>) {
    let specular_file = File::open("assets/2k_earth_specular_map.tif");
    if let Err(_) = specular_file {
        println!("specular-texture not found");
        return (None, None);
    }
    let specular_file = specular_file.unwrap();
    let mut decoder = Decoder::new(specular_file).unwrap();

    let specular_pixels = if let DecodingResult::U8(v) = decoder.read_image().unwrap() {
        v
    } else {
        return (None, None);
    };

    let mut specular_data = vec![0; w * h];
    for i in 0..w * h {
        specular_data[i] = specular_pixels[3 * i];
    }

    let normal_file = File::open("assets/2k_earth_normal_map.tif");
    if let Err(_) = normal_file {
        println!("normal-texture not found");
        return (Some(specular_data), None);
    }
    let normal_file = normal_file.unwrap();
    decoder = Decoder::new(normal_file).unwrap();

    let normal_pixels = if let DecodingResult::U8(v) = decoder.read_image().unwrap() {
        v
    } else {
        return (Some(specular_data), None);
    };

    let mut normal_data = vec![Vec3::zero(); w * h];
    for i in 0..w * h {
        let r = normal_pixels[3 * i] as f64 / 255.;
        let g = normal_pixels[3 * i + 1] as f64 / 255.;
        let b = normal_pixels[3 * i + 2] as f64 / 255.;
        normal_data[i] = Vec3(r * 2. - 1., g * 2. - 1., b * 2. - 1.);
    }

    (Some(specular_data), Some(normal_data))
}
