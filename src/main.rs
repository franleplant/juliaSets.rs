extern crate image;
extern crate num;
use std::fs::File;
use std::path::Path;
use std::ops::Add;
use image::{ImageBuffer, Rgba};
use num::{Complex};

static MAX_ITER: i64 = 1000;
static BAILOUT: f64 = 2.0;
static N: i64 = 100;
static M: i64 = 100;


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

fn fractal(z0: &Complex<f64>, delta: &(f64, f64), width: i64, height: i64) {
    let c  = Complex::new( 0.279, 0.0 );

    let img = ImageBuffer::from_fn(width as u32, height as u32, |i, j| {
        let mut z = Complex::new(
            z0.re + delta.0 * (i as f64),
            z0.im - delta.1 * (j as f64)
        );

        let mut k = 0;
        while (z.norm_sqr() < BAILOUT) && (k < MAX_ITER) {
            z = z * z + c;

            k += 1;
        }
        println!("k {}", k);

        Rgba([((k*10) % 255) as u8,255u8,255u8,255u8])
    });
    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageRgba8(img).save(fout, image::PNG);
}

fn main() {
    let center = Complex::new(0.0, 0.0);
    let range_x = 2.;
    let range_y = 2.;
    let z0 = get_z0(&center, range_x, range_y);
    let delta = get_delta(range_x, range_y, N, M);
    fractal(&z0, &delta, N, M);
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
