extern crate image;
extern crate num;
extern crate conrod;
extern crate gif;
extern crate rayon;
#[macro_use]
extern crate clap;

mod colorizer;
mod generator;

use std::fs::File;
use std::default::Default;
use std::path::Path;

use num::Complex;
use clap::{App, ArgMatches};

use colorizer::SimpleColorizer;
use generator::Generator;


//TODO support for more functions
//TODO support for gifs
fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();


    if let Err(e) = run(matches) {
        println!("Error {}", e);

        ::std::process::exit(1);
    }
}

fn run(matches: ArgMatches) -> Result<(), String> {
    let file = matches.value_of("INPUT");
    if file == None {
        //TODO generate random names when no output file is present
        return Err(format!("No output filename present"));
    }

    let mut gen: Generator<SimpleColorizer> = Default::default();

    let parallel = matches.value_of("parallel");
    if parallel != None {
        let parallel = parallel.unwrap().parse::<bool>().map_err(|_| {
            format!("Error parsing parallel")
        })?;
        gen.parallel = parallel;
    }

    let max_iter = matches.value_of("max_iter");
    if max_iter != None {
        let max_iter = max_iter.unwrap().parse::<u32>().map_err(|_| {
            format!("Error parsing max_iter")
        })?;
        gen.max_iter = max_iter;
    }

    let width = matches.value_of("width");
    if width != None {
        let width = width.unwrap().parse::<u32>().map_err(|_| {
            format!("Error parsing width")
        })?;
        gen.width = width;
    }

    let height = matches.value_of("height");
    if height != None {
        let height = height.unwrap().parse::<u32>().map_err(|_| {
            format!("Error parsing height")
        })?;
        gen.height = height;
    }

    let zoom = matches.value_of("zoom");
    if zoom != None {
        let zoom = zoom.unwrap().parse::<f64>().map_err(|_| {
            format!("Error parsing zoom")
        })?;
        gen.zoom = zoom;
    }

    let mut center: Complex<f64> = Complex::new(0.0, 0.0);
    let center_x = matches.value_of("center_x");
    if center_x != None {
        let center_x = center_x.unwrap().parse::<f64>().map_err(|_| {
            format!("Error parsing center_x")
        })?;
        center.re = center_x;
    }

    let center_y = matches.value_of("center_y");
    if center_y != None {
        let center_y = center_y.unwrap().parse::<f64>().map_err(|_| {
            format!("Error parsing center_y")
        })?;
        center.im = center_y;
    }

    gen.center = center;

    let mut constant: Complex<f64> = Complex::new(-0.4, 0.6);
    let constant_x = matches.value_of("constant_x");
    if constant_x != None {
        let constant_x = constant_x.unwrap().parse::<f64>().map_err(|_| {
            format!("Error parsing constant_x")
        })?;
        constant.re = constant_x;
    }

    let constant_y = matches.value_of("constant_y");
    if constant_y != None {
        let constant_y = constant_y.unwrap().parse::<f64>().map_err(|_| {
            format!("Error parsing constant_y")
        })?;
        constant.im = constant_y;
    }

    gen.constant = constant;

    let kind_fn = matches.value_of("kind_fn");
    if kind_fn != None {
        let kind_fn = kind_fn.unwrap().parse::<usize>().map_err(|_| {
            format!("Error parsing kind_fn")
        })?;
        gen.kind_fn = kind_fn;
    }

    println!("Settings {}", gen.settings_to_string());

    let img = gen.render();
    let ref mut fout = File::create(&Path::new(&file.unwrap())).unwrap();
    let _ = image::ImageRgba8(img).save(fout, image::PNG);

    Ok(())
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
