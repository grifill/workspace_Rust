// Crates: 
// Num
extern crate num;
use num::Complex;

fn main() {
    println!("Hello, Mandelbrot!");
}

/// Try to determine whether 'c' belongs to the Mandelbrot set, limiting the number of iterations 'limit'
/// If 'c' does not belong to the Mandelbrot set, return 'Some(i)', where 'i' - number of iterations
/// If 'c' can belong to the Mandelbrot set, return 'None'
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> { 
    let mut z = Complex { re: 0.0, im: 0.0 }; 
    for i in 0..limit { 
        z = z*z + c; 
        if z.norm_sqr() > 4.0 { 
            return Some(i); 
        } 
    } 
    None
}
