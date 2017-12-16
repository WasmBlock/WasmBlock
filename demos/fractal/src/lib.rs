#[macro_use(wasmblock_setup)]
extern crate wasmblock;

//using special macros for global state, see below
use std::mem;
use std::os::raw::{c_char,c_void};
use std::ffi::{CString};
use wasmblock::{dom,canvas,console};
use std::ops::{Add, Mul};


// needed for allocation and deallocation functions
wasmblock_setup!();

#[derive(Copy, Clone)]
struct Complex {
    re: f64,
    im: f64
}

impl Add<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn add(self, other: Complex) -> Complex {
        Complex::new(self.re + other.re, self.im + other.im)
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn mul(self, other: Complex) -> Complex {
        let re = self.re.clone() * other.re.clone() - self.im.clone() * other.im.clone();
        let im = self.re * other.im + self.im * other.re;
        Complex::new(re, im)
    }
}

impl Complex {
    #[inline]
    pub fn new(re: f64, im: f64) -> Complex {
        Complex { re: re, im: im }
    }

    #[inline]
    pub fn norm_sqr(&self) -> f64 {
        self.re.clone() * self.re.clone() + self.im.clone() * self.im.clone()
    }
}

fn escape_time(c: Complex, limit: u32) -> Option<u32> {
        let mut z = Complex { re: 0.0, im: 0.0};
        for i in 0..limit {
            z = z*z + c;
            if z.norm_sqr() > 4.0 {
                return Some(i);
            }
        }
        None
}

fn pixel_to_point(bounds:(usize,usize),pixel:(usize,usize),upper_left:Complex,lower_right:Complex) -> Complex {
    let (width,height) = (lower_right.re - upper_left.re, lower_right.im - upper_left.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im + pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[no_mangle]
pub fn start() -> () {
    console::time();
    dom::create_element("body","style","game_styles");
    dom::set_inner_html("#game_styles",include_str!("fractal.css"));
    dom::create_element("body","canvas","screen");
    dom::set_attribute("#screen","width","600");
    dom::set_attribute("#screen","height","400");
    let ctx = canvas::get_context("#screen");
    let upper_left = Complex{re:-1.2,im:0.35};
    let lower_right = Complex{re:-1.0,im:0.2};
    let bounds = (600,400);
    for column in 0..bounds.0 {
        for row in 0..bounds.1 {
            let point = pixel_to_point(bounds,(column,row),upper_left,lower_right);
            let v = match escape_time(point, 255) {
                None => 0,
                Some(count) => (255 - count) as u8
            };
            canvas::set_fill_style_color(ctx,v,v,v,1.0);
            canvas::fill_rect(ctx,column as f32,row as f32,1.0,1.0);
        }
    }
    console::time_end();
}
