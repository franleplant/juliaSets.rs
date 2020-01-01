extern crate image;
extern crate num;
extern crate conrod;
extern crate gif;
extern crate rayon;
#[macro_use]
extern crate clap;

mod fparams;
mod fgenerator;
mod colorizer;
mod funcs;

use std::fs::File;
use std::path::Path;

use clap::App;

use fparams::FParams;
use fgenerator::FGenerator;


//TODO add travis
//TODO create binaries for different OSs?
//TODO add more test cases
//TODO support for a parametric colorizer
//TODO support for more functions
//TODO try to refactor the func selectors and the func stringifier
fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let params = FParams::from(&matches);

    let file_ext = if params.is_gif { "gif" } else { "png" };
    let file_name = matches.value_of("INPUT")
        .unwrap_or(&format!("{}.{}", "test", file_ext)).to_string();

    println!("Creating a fractal with this settings...\n\n");
    println!("{}", params);
    let mut gen = FGenerator::new(params);

    let img = gen.render();
    let ref mut fout = File::create(&Path::new(&file_name)).unwrap();
    let _ = image::ImageRgba8(img).save(fout, image::PNG);

    if !gen.params.is_gif {
        let img = gen.render();
        let ref mut fout = File::create(&Path::new(&file_name)).unwrap();
        let _ = image::ImageRgba8(img).save(fout, image::PNG);

    } else {
        use gif::{Frame, Encoder, Repeat, SetParameter};

        let ref mut gif = File::create(&Path::new(&file_name)).unwrap();
        let mut encoder = Encoder::new(&mut *gif, gen.params.width as u16, gen.params.height as u16, &[]).unwrap();
        encoder.set(Repeat::Infinite).unwrap();

        for _ in 0..400 {
            let new_zoom = gen.params.zoom * gen.params.zoom_speed;
            gen.params.set_zoom(new_zoom);
            let mut img = gen.render();
            // Create frame from data
            let frame = Frame::from_rgba(gen.params.width as u16, gen.params.height as u16, &mut *img);

            // Write frame to file
            encoder.write_frame(&frame).unwrap();
        }

    }
}
