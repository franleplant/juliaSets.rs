use std::marker::Sync;
use std::default::Default;
use std::fmt::Debug;
use image;

//TODO avoid code duplication for the parallel stuff, maybe macros?
use rayon::prelude::*;
use num::Complex;

pub trait Colorizer {
    fn calc_color(&self, k: u32) -> Vec<u8>;
}


#[derive(Debug)]
pub struct Generator<C: Colorizer + Sync + Debug> {
    pub bailout: f64,
    pub max_iter: u32,
    pub width: u32,
    pub height: u32,
    pub range_x: f64,
    pub range_y: f64,
    pub zoom: f64,
    pub colorizer: C,
    pub center: Complex<f64>,
    pub parallel: bool,
    pub constant: Complex<f64>,
    pub kind_fn: usize,
}

impl<C: Default + Colorizer + Sync + Debug> Default for Generator<C> {
    fn default() -> Generator<C> {
        let bailout = 2.0;
        let max_iter = 100;
        let width = 1000;
        let height = 1000;
        let range_x = 3.0;
        let range_y = 3.0;
        let zoom = 1.0;

        Generator {
            bailout: bailout,
            max_iter: max_iter,
            width: width,
            height: height,
            range_x: range_x,
            range_y: range_y,
            zoom: zoom,
            center: Complex::new(0.0, 0.0),
            constant: Complex::new(0.0, 0.0),
            colorizer: Default::default(),
            parallel: false,
            kind_fn: 0,
        }
    }
}

impl<C: Colorizer + Sync + Debug> Generator<C> {
    fn calc_z0(&self) -> Complex<f64> {
        let scaled_range_x = self.range_x / self.zoom;
        let scaled_range_y = self.range_y / self.zoom;

        Complex::new(
            self.center.re - scaled_range_x / 2.0,
            self.center.im + scaled_range_y / 2.0,
        )
    }

    fn calc_delta(&self) -> Complex<f64> {
        let scaled_range_x = self.range_x / self.zoom;
        let scaled_range_y = self.range_y / self.zoom;

        Complex::new(
            scaled_range_x / self.width as f64,
            scaled_range_y / self.height as f64,
        )
    }

    pub fn render(&self) -> image::RgbaImage {
        let z0 = self.calc_z0();
        let delta = self.calc_delta();


        let mut empty_image: Vec<u8> = vec![];
        for _ in 0..self.width * self.height {
            empty_image.push(0);
        }

        let raw = if self.parallel {
            empty_image
                .par_iter()
                .enumerate()
                .map(|(k, _)| self.into_2d(k))
                .flat_map(|(i, j)| {
                    let mut z =
                        Complex::new(z0.re + delta.re * (i as f64), z0.im - delta.im * (j as f64));

                    let mut k = 0;
                    while (z.norm_sqr() < self.bailout) && (k < self.max_iter) {
                        z = self.call_fn(z);

                        k += 1;
                    }

                    self.colorizer.calc_color(k)
                })
                .collect()
        } else {
            empty_image
                .iter()
                .enumerate()
                .map(|(k, _)| self.into_2d(k))
                .flat_map(|(i, j)| {
                    let mut z =
                        Complex::new(z0.re + delta.re * (i as f64), z0.im - delta.im * (j as f64));

                    let mut k = 0;
                    while (z.norm_sqr() < self.bailout) && (k < self.max_iter) {
                        z = self.call_fn(z);

                        k += 1;
                    }

                    self.colorizer.calc_color(k)
                })
                .collect()
        };

        image::ImageBuffer::from_vec(self.width, self.height, raw).unwrap()
    }

    fn into_2d(&self, k: usize) -> (usize, usize) {
        let i = k % self.width as usize;
        let j = k / self.width as usize;
        (i, j)
    }

    fn call_fn(&self, x: Complex<f64>) -> Complex<f64> {
        match self.kind_fn {
            0 => self.square(x),
            _ => panic!("Function not found"),
        }
    }

    fn square(&self, x: Complex<f64>) -> Complex<f64> {
        x * x + self.constant
    }

    pub fn settings_to_string(&self) -> String {
        vec![
            //format!("bailout = {}", self.bailout)
            format!("max_iter = {}", self.max_iter),
            format!("width = {}", self.width),
            format!("height = {}", self.height),
            format!("zoom = {}", self.zoom),
            format!("center = {}", self.center),
            format!("parallel = {}", self.parallel),
            format!("constant = {}", self.constant),
        ].join(" ")
    }
}


//TODO
#[test]
fn test_get_z0() {
    let center = Complex::new(0.0, 0.0);
    let z0 = get_z0(&center, 2.0, 2.0);
    assert!(z0.re == -1.0);
    assert!(z0.im == 1.0);
}

#[test]
fn test_get_delta() {
    let (dx, dy) = get_delta(2.0, 2.0, 2, 2);
    assert!(dx == 1.0);
    assert!(dy == 1.0);
}
