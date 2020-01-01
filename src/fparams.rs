use std::default::Default;
use std::fmt;
use std::str::FromStr;

use num::Complex;
use clap::ArgMatches;

#[derive(Debug)]
pub struct FParams {
    pub bailout: f64,
    pub max_iter: u32,
    pub width: u32,
    pub height: u32,
    pub range_x: f64,
    pub range_y: f64,
    pub zoom: f64,
    pub center: Complex<f64>,
    pub parallel: bool,
    pub constant: Complex<f64>,
    pub z0: Complex<f64>,
    pub delta: Complex<f64>,
    pub kind_fn: usize,
    pub colorizer: usize,
    pub is_gif: bool,
    pub zoom_speed: f64,
    pub gif_frames: u64,
}

impl Default for FParams {
    fn default() -> FParams {
        let mut params = FParams {
            bailout: 2.0,
            max_iter: 500,
            width: 1000,
            height: 1000,
            range_x: 3.0,
            range_y: 3.0,
            zoom: 1.0,
            center: Complex::new(0.0, 0.0),
            constant: Complex::new(0.0, 0.0),
            z0: Complex::new(0.0, 0.0),
            delta: Complex::new(0.0, 0.0),
            colorizer: 0,
            parallel: false,
            kind_fn: 0,
            is_gif: false,
            zoom_speed: 1.1,
            gif_frames: 400,
        };

        params.calc_z0();
        params.calc_delta();
        params
    }
}

impl FParams {
    fn calc_z0(&mut self) {
        let scaled_range_x = self.range_x / self.zoom;
        let scaled_range_y = self.range_y / self.zoom;

        self.z0 = Complex::new(
            self.center.re - scaled_range_x / 2.0,
            self.center.im + scaled_range_y / 2.0,
        );
    }

    fn calc_delta(&mut self) {
        let scaled_range_x = self.range_x / self.zoom;
        let scaled_range_y = self.range_y / self.zoom;

        self.delta = Complex::new(
            scaled_range_x / self.width as f64,
            scaled_range_y / self.height as f64,
        );
    }

    pub fn set_zoom(&mut self, zoom: f64) {
        self.zoom = zoom;
        self.calc_z0();
        self.calc_delta();
    }
}

fn parse_with_default<'a, T>(opt: Option<&'a str>, default: T) -> T
where
    T: FromStr,
{
    if opt.is_none() {
        return default;
    }

    opt.unwrap().parse().unwrap_or(default)
}

impl<'a> From<&'a ArgMatches<'a>> for FParams {
    fn from(matches: &ArgMatches) -> FParams {
        let FParams {
            max_iter,
            width,
            height,
            range_x,
            range_y,
            zoom,
            center,
            constant,
            kind_fn,
            zoom_speed,
            gif_frames,

            ..
        } = Default::default();

        let center = Complex::new(
            parse_with_default(matches.value_of("center_x"), center.re),
            parse_with_default(matches.value_of("center_y"), center.im),
        );

        let constant = Complex::new(
            parse_with_default(matches.value_of("constant_x"), constant.re),
            parse_with_default(matches.value_of("constant_y"), constant.im),
        );

        let mut params = FParams {
            max_iter: parse_with_default(matches.value_of("max_iter"), max_iter),
            width: parse_with_default(matches.value_of("width"), width),
            height: parse_with_default(matches.value_of("height"), height),
            range_x: parse_with_default(matches.value_of("range_x"), range_x),
            range_y: parse_with_default(matches.value_of("range_y"), range_y),
            zoom: parse_with_default(matches.value_of("zoom"), zoom),
            center: center,
            parallel: matches.is_present("parallel"),
            constant: constant,
            kind_fn: parse_with_default(matches.value_of("kind_fn"), kind_fn),
            is_gif: matches.is_present("is_gif"),
            zoom_speed: parse_with_default(matches.value_of("zoom_speed"), zoom_speed),
            gif_frames: parse_with_default(matches.value_of("gif_frames"), gif_frames),

            ..Default::default()
        };

        params.calc_z0();
        params.calc_delta();
        params
    }
}


impl fmt::Display for FParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let func = match self.kind_fn {
            0 => "x^2 + c",
            1 => "x^3 + c",
            10 => "exp(z) + c",
            100 => "(1 - z^3 / 6) / (z - z^2 / 2)^2 + c",
            200 => "[(z2+z)/Ln(z)] + c",
            _ => "default",
        };

        let s = vec![
            "Measures".to_string(),
            "=======".to_string(),
            format!("{:<15} {}", "width", self.width),
            format!("{:<15} {}", "height", self.height),
            format!("{:<15} {}, {}", "range", self.range_x, self.range_y),
            format!("{:<15} {}", "zoom", self.zoom),
            format!("{:<15} {}", "center", self.center),
            format!("{:<15} {}", "z0 (calc)", self.z0),
            String::new(),

            "Iteration params".to_string(),
            "================".to_string(),
            format!("{:<15} {}", "max iter", self.max_iter),
            format!(
                "{:<15} {}, {}",
                "delta (calc)",
                self.delta.re,
                self.delta.im
            ),
            format!("{:<15} {}", "parallel", self.parallel),
            format!("{:<15} {}", "is gif", self.is_gif),
            format!("{:<15} {}", "zoom speed", self.zoom_speed),
            format!("{:<15} {}", "gif frames", self.gif_frames),
            String::new(),

            "Function".to_string(),
            "========".to_string(),
            format!("{:<15} {}", "kind fn", func),
            format!("{:<15} {}", "constant", self.constant),
            format!("{:<15} {}", "bailout", self.bailout),
            String::new(),
        ].join("\n");

        write!(f, "{}", s)
    }
}
