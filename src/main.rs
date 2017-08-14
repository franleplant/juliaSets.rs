extern crate image;
extern crate num;
extern crate conrod;
extern crate gif;
extern crate rayon;

mod colorizer;
mod generator;

use std::fs::File;
use std::default::Default;
use std::path::Path;
use num::Complex;

use colorizer::SimpleColorizer;
use generator::Generator;




/// create a png
fn main_png() {
    let mut g: Generator<SimpleColorizer> = Default::default();
    //g.center = Complex::new(-0.8195999999999999, 0.9);
    let c = Complex::new(-0.4, 0.6);
    let f = move |z: Complex<f64>| z * z + c;
    let img = g.render(f);
    let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();
    let _ = image::ImageRgba8(img).save(fout, image::PNG);
}



fn main() {

    main_png();
    //main_gif();

}



// Create a gif
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
