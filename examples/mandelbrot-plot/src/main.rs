// Crates: 
// Num
extern crate num;
use num::Complex;

// Str
use std::str::FromStr;

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

/// Parse string 's', which contains a pair of coordinates, for example: `"400x600"` or '"(1.0, 0.5)"'
/// 's' must have the form <left><sep><right>, where: 
/// <sep> is the character specified in the 'separator' argument, 
/// <left> and <right> are strings that can be parsed using the 'T::from_str' method
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) { 
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}