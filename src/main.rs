extern crate image;
extern crate num;
extern crate conrod;
extern crate gif;

mod colorizer;

use colorizer::SimpleColorizer;
use std::fs::File;
use std::default::Default;
use std::path::Path;
use num::Complex;

pub trait Colorizer {
    fn calc_color(&self, k: u32) -> Vec<u8>;
}


struct ImgGenerator<C: Colorizer> {
    bailout: f64,
    max_iter: u32,
    width: u32,
    height: u32,
    range_x: f64,
    range_y: f64,
    zoom: f64,
    colorizer: C,
    center: Complex<f64>,
}

impl<C: Default + Colorizer> Default for ImgGenerator<C> {
    fn default() -> ImgGenerator<C> {
        let bailout = 2.0;
        let max_iter = 100;
        let width = 1000;
        let height = 1000;
        let range_x = 3.0;
        let range_y = 3.0;
        let zoom = 1.0;

        ImgGenerator {
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

impl<C: Colorizer> ImgGenerator<C> {
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
        F: Fn(Complex<f64>) -> Complex<f64>,
    {
        let z0 = self.calc_z0();
        let delta = self.calc_delta();


        let mut empty_image: Vec<u8> = vec![];
        for _ in 0..self.width * self.height {
            empty_image.push(0);
        }

        let raw = empty_image
            .iter()
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



/// create a png
fn main_png() {
    let mut g: ImgGenerator<SimpleColorizer> = Default::default();
    //g.center = Complex::new(-0.8195999999999999, 0.9);
    let c = Complex::new(-0.4, 0.6);
    let f = move |z: Complex<f64>| z * z + c;
    let img = g.render(f);
    let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();
    let _ = image::ImageRgba8(img).save(fout, image::PNG);
}


/// Create a gif
//fn main_gif() {
//let c = Complex::new( 0.279, 0.0 );
//let f = |z| z*z + c;
//let center = Complex::new(0.4959986657096176, 0.17224325099767768);
//let colorizer = color_by_index_factory(STEP as f32, PHASE as f32, 0.7f32, 0.6f32, 0.9f32 );


//let ref mut gif = File::create(&Path::new("fractal.gif")).unwrap();
//// Create encoder
//let encoder = gif::Encoder::new(&mut *gif, N as u16, M as u16);
//// Write header to file
//let mut encoder = encoder.write_global_palette(&[]).unwrap();

//let mut range_x = RANGE_X;
//let mut range_y = RANGE_Y;
//for _ in 1..400 {
//range_x *= 0.99;
//range_y *= 0.99;
//let z0 = get_z0(&center, range_x, range_y);
//let delta = get_delta(range_x, range_y, N, M);
//let mut img = fractal(&f, &z0, &delta, N, M, &colorizer, MAX_ITER);

//// Create frame from data
//let frame = gif::Frame::from_rgba(N as u16, M as u16, &mut *img);

//// Write frame to file
//encoder.write_frame(&frame).unwrap();
//}
//}

fn main() {

    main_png();
    //main_gif();

}


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


#[test]
fn test_color_by_index_factory() {
    let f = color_by_index_factory(1.0 as f32, 0.0 as f32, 0.7f32, 0.6f32, 0.9f32);
    let c = f(2 as f32);
}
