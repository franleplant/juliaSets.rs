use image;
use num::Complex;
use rayon::prelude::*;

use fparams::FParams;
use colorizer::SimpleColorizer;
use funcs;

pub type ComplexFn = fn(Complex<f64>, Complex<f64>) -> Complex<f64>;

pub trait Colorizer {
    fn calc_color(&self, k: u32) -> Vec<u8>;
}


pub struct FGenerator {
    pub params: FParams,
    func: ComplexFn,
    //TODO improve this interface
    //we need to be able to parameterize SimpleColorizer but also to use other colorizer methods
    colorizer: SimpleColorizer,
}

impl FGenerator {
    pub fn new(params: FParams) -> FGenerator {
        let func = match params.kind_fn {
            0 => funcs::x2,
            1 => funcs::x3,
            10 => funcs::q1,
            20 => funcs::m1,
            _ => panic!("Function not supported"),
        };


        FGenerator {
            params: params,
            func: func,
            colorizer: Default::default(),
        }
    }

    fn get_params(&self) -> &FParams {
        &self.params
    }

    pub fn render(&self) -> image::RgbaImage {
        let params = self.get_params();

        let mut empty_image: Vec<u8> = vec![];
        for _ in 0..params.width * params.height {
            empty_image.push(0);
        }

        let raw = if params.parallel {
            empty_image
                .par_iter()
                .enumerate()
                .map(|(k, _)| self.into_2d(k))
                .flat_map(|(i, j)| self.escape_time_to_color(i, j))
                .collect()
        } else {
            empty_image
                .iter()
                .enumerate()
                .map(|(k, _)| self.into_2d(k))
                .flat_map(|(i, j)| self.escape_time_to_color(i, j))
                .collect()
        };

        image::ImageBuffer::from_vec(params.width, params.height, raw).unwrap()
    }

    fn into_2d(&self, k: usize) -> (usize, usize) {
        let params = self.get_params();
        let i = k % params.width as usize;
        let j = k / params.height as usize;
        (i, j)
    }

    fn escape_time_to_color(&self, i: usize, j: usize) -> Vec<u8> {
        let params = self.get_params();
        let mut z = Complex::new(
            params.z0.re + params.delta.re * (i as f64),
            params.z0.im - params.delta.im * (j as f64),
        );

        let mut k = 0;
        while (z.norm_sqr() < params.bailout) && (k < params.max_iter) {
            z = (self.func)(z, params.constant);

            k += 1;
        }

        self.colorizer.calc_color(k)
    }
}
