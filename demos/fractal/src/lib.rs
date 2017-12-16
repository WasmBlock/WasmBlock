#[macro_use(wasmblock_setup)]
extern crate wasmblock;

//using special macros for global state, see below
use std::mem;
use std::os::raw::{c_char,c_void};
use std::ffi::{CString};
use wasmblock::{dom,canvas};
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

#[no_mangle]
pub fn start() -> () {
    dom::create_element("body","style","game_styles");
    dom::set_inner_html("#game_styles",include_str!("fractal.css"));
    dom::create_element("body","canvas","screen");
    dom::set_attribute("#screen","width","600");
    dom::set_attribute("#screen","height","400");
    let ctx = canvas::get_context("#screen");
    for x in 0..600 {
        for y in 0..400 {
            let r = ((x as f32)/600.0 * 255.0) as u8;
            let g = ((y as f32)/600.0 * 255.0) as u8;
            let b = 0;
            let a = 1.0;
            canvas::set_fill_style_color(ctx,r,g,b,a);
            canvas::fill_rect(ctx,x as f32,y as f32,1.0,1.0);
        }
    }
}
