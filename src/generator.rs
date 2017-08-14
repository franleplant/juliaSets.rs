use std::marker::Sync;
use std::default::Default;
use image;

use rayon::prelude::*;
use num::Complex;

pub trait Colorizer {
    fn calc_color(&self, k: u32) -> Vec<u8>;
}


pub struct Generator<C: Colorizer + Sync> {
   pub bailout: f64,
   pub max_iter: u32,
   pub width: u32,
   pub height: u32,
   pub range_x: f64,
   pub range_y: f64,
   pub zoom: f64,
   pub colorizer: C,
   pub center: Complex<f64>,
}

impl<C: Default + Colorizer + Sync> Default for Generator<C> {
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
            colorizer: Default::default(),
        }
    }
}

impl<C: Colorizer + Sync> Generator<C> {
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

    pub fn render<F>(&self, func: F) -> image::RgbaImage
    where
        F: Fn(Complex<f64>) -> Complex<f64> + Sync,
    {
        let z0 = self.calc_z0();
        let delta = self.calc_delta();


        let mut empty_image: Vec<u8> = vec![];
        for _ in 0..self.width * self.height {
            empty_image.push(0);
        }

        let raw = empty_image
            .par_iter()
            .enumerate()
            .map(|(k, _)| {
                let i = k % self.width as usize;
                let j = k / self.width as usize;
                (i, j)
            })
            .flat_map(|(i, j)| {
                let mut z =
                    Complex::new(z0.re + delta.re * (i as f64), z0.im - delta.im * (j as f64));

                let mut k = 0;
                while (z.norm_sqr() < self.bailout) && (k < self.max_iter) {
                    z = func(z);

                    k += 1;
                }

                self.colorizer.calc_color(k)
            })
            .collect();

        image::ImageBuffer::from_vec(self.width, self.height, raw).unwrap()
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
