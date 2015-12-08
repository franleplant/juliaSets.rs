extern crate image;
extern crate num;
extern crate conrod;
extern crate elmesque;
extern crate gif;

use std::fs::File;
use std::path::Path;
use num::{Complex};
use conrod::color::{Color};
use std::f64::consts::PI;

static MAX_ITER: i64 = 1000;
static N: i64 = 1000;
static M: i64 = 1000;
static RANGE_X: f64 = 3.;
static RANGE_Y: f64 = 3.;
const STEP: f32 = 5.0;
const PHASE: f32 = 0.0;


type Colorizer = Fn(f32) -> image::Rgba<u8>;
fn get_z0(center: &Complex<f64>, range_x: f64, range_y: f64 ) -> Complex<f64> {
    Complex::new(
        center.re - range_x / 2.0,
        center.im + range_y / 2.0
    )
}

fn get_delta(range_x: f64, range_y: f64, width: i64, height: i64) -> (f64, f64) {
    (
        range_x / (width as f64),
        range_y / (height as f64)
    )
}


fn color_by_index_factory(step: f32, phase: f32, saturation: f32, lightness: f32, alpha: f32) -> Box<Colorizer> {
    let conv_factor: f32 = 2.0f32 * PI as f32/ 360.0f32;

    Box::new(move |k: f32| {
        let hue = ((step * k + phase) % 360.0f32) * conv_factor;
        let elmesque::color::Rgba(r, g, b, a) = Color::Hsla(hue, saturation, lightness, alpha).to_rgb();
        let r = (r * 255f32) as u8;
        let g = (g * 255f32) as u8;
        let b = (b * 255f32) as u8;
        let a = (a * 255f32) as u8;
        image::Rgba([r, g, b, a])
    })
}

fn color_by_index(k: i64) -> image::Rgba<u8> {
    let hue = ((STEP * (k as f32) + PHASE)% 360.0) * 2.0 * PI as f32 / 360.0;
    //println!("k {}, hue {}", k, hue);
    let color = Color::Hsla(hue as f32, 0.7f32, 0.6f32, 0.9f32);
    let elmesque::color::Rgba(r, g, b, a) = color.to_rgb();
    let r = r * 255f32;
    let g = g * 255f32;
    let b = b * 255f32;
    let a = a * 255f32;
    //println!("{}, {}, {}, {}", r,g,b,a);
    image::Rgba([r as u8, g as u8, b as u8, a as u8])
}


fn fractal<F>(f: &F, z0: &Complex<f64>, delta: &(f64, f64), width: i64, height: i64, colorizer: &Box<Colorizer>, max_iter: i64) -> image::RgbaImage
    where F: Fn(Complex<f64>) -> Complex<f64> {

    static mut zmax: Complex<f64> = Complex{ re:0.0, im: 0.0};
    static mut kmax: i64 = 0;
    static BAILOUT: f64 = 2.0;

    let img = image::ImageBuffer::from_fn(width as u32, height as u32, |i, j| {
        let z_original = Complex::new(
            z0.re + delta.0 * (i as f64),
            z0.im - delta.1 * (j as f64)
        );

        let mut z = z_original;

        let mut k = 0;
        while (z.norm_sqr() < BAILOUT) && (k < max_iter) {
            z = f(z);

            k += 1;
        }

        if cfg!(debug_assertions) {
            unsafe {
                if k > kmax {
                    kmax = k;
                    zmax = z_original;
                }
            }
        }
        colorizer(k as f32)
    });


    if cfg!(debug_assertions) {
        unsafe {
            println!("deepest point {}, {} kmax {}", zmax.re, zmax.im, kmax);
        }
    }

    img
}

fn main() {
    // The following commented lines are for saving a single "frame" into a png
    //let center = Complex::new(0.0, 0.0);
    //let z0 = get_z0(&center, RANGE_X, RANGE_Y);
    //let delta = get_delta(RANGE_X, RANGE_Y, N, M);
    //let f = |z| z*z + Complex::new( 0.279, 0.0 );
    //let mut img = fractal(f, &z0, &delta, N, M, MAX_ITER);
    // Save the image as “fractal.png”
    //let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();
    // We must indicate the image’s color type and what format to save as
    //let _ = image::ImageRgba8(img).save(fout, image::PNG);

    let c = Complex::new( 0.279, 0.0 );
    let f = |z| z*z + c;
    let center = Complex::new(0.4959986657096176, 0.17224325099767768);
    let colorizer = color_by_index_factory(STEP as f32, PHASE as f32, 0.7f32, 0.6f32, 0.9f32 );


    let ref mut gif = File::create(&Path::new("fractal.gif")).unwrap();
    // Create encoder
    let encoder = gif::Encoder::new(&mut *gif, N as u16, M as u16);
    // Write header to file
    let mut encoder = encoder.write_global_palette(&[]).unwrap();

    let mut range_x = RANGE_X;
    let mut range_y = RANGE_Y;
    for _ in 1..400 {
        range_x *= 0.99;
        range_y *= 0.99;
        let z0 = get_z0(&center, range_x, range_y);
        let delta = get_delta(range_x, range_y, N, M);
        let mut img = fractal(&f, &z0, &delta, N, M, &colorizer, MAX_ITER);

        // Create frame from data
        let frame = gif::Frame::from_rgba(N as u16, M as u16, &mut *img);

        // Write frame to file
        encoder.write_frame(&frame).unwrap();
    }
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
    let f = color_by_index_factory(1.0 as f32, 0.0 as f32, 0.7f32, 0.6f32, 0.9f32 );
    let c = f(2 as f32);
}
