// Crates: 
// Num
extern crate num;
use num::Complex;

// Image
extern crate image; 
use image::ColorType; 
use image::png::PNGEncoder; 
use std::fs::File;

// CrossBeam
extern crate crossbeam;

// Str
use std::str::FromStr;

use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 5 { 
        writeln!(std::io::stderr(), "FILE PIXELS UPPERLEFT LOWERRIGHT")
        .unwrap();
        writeln!(std::io::stderr(), "{} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0])
        .unwrap(); 
        std::process::exit(1); 
    }
    
    let bounds = parse_pair(&args[2], 'x').expect("Error image size"); 
    let upper_left = parse_complex(&args[3]).expect("Error left corner point"); 
    let lower_right = parse_complex(&args[4]).expect("Right left corner point"); 
    let mut pixels = vec![0; bounds.0 * bounds.1];
    //render(&mut pixels, bounds, upper_left, lower_right);
    
    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;
    
    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
                spawner.spawn(move || {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        });
}   
    write_image(&args[1], &pixels, bounds).expect("Error writing PNG-file");
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

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",         ','), None);
    assert_eq!(parse_pair::<i32>("10,",      ','), None);
    assert_eq!(parse_pair::<i32>(",10",      ','), None);
    assert_eq!(parse_pair::<i32>("10,20",    ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy",  ','), None);
    assert_eq!(parse_pair::<f64>("0.5x",     'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5",  'x'), Some((0.5, 1.5)));
}

/// Parses a pair of floating-point numbers separated by a comma and returns it as a complex number
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), 
    Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);
}


/// Given the row and column of a pixel in the output image, 
/// returns the corresponding point in the complex plane. 
/// 'bounds' is a pair defining the width and height of the image in pixels. 
/// 'pixel' is a (row, column) pair defining a specific pixel in the image. 
/// The 'upper_left' and 'lower_right' parameters are points in the complex plane describing the area covered by the image
fn pixel_to_point(bounds: (usize, usize), 
                  pixel: (usize, usize), 
                  upper_left: Complex<f64>, 
                  lower_right: Complex<f64>)
                  -> Complex<f64> {
                    
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);
    Complex {
        // pixel.1 increases as you move down, while the imaginary part increases as you move up
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64 
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 100), (25, 75), 
                     Complex { re: -1.0, im: 1.0 }, 
                     Complex { re: 1.0, im: -1.0 }), 
                     Complex { re: -0.5, im: -0.5 });
}

/// Draws a rectangular portion of the Mandelbrot set into the pixel buffer
/// The 'bounds' argument specifies the width and height of the 'pixels' buffer, in which each byte represents one grayscale pixel 
/// The 'upper_left' and 'lower_right' arguments define the points in the complex plane 
/// corresponding to the upper-left and lower-right corners of the pixel buffer
fn render(pixels: &mut [u8], 
          bounds: (usize, usize), 
          upper_left: Complex<f64>, 
          lower_right: Complex<f64>) { 
    
    assert!(pixels.len() == bounds.0 * bounds.1);
    for row in 0 .. bounds.1 { 
        for column in 0 .. bounds.0 { 
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right); 
            pixels[row * bounds.0 + column] = match escape_time(point, 255) { None => 0, Some(count) => 255 - count as u8}; 
        }
    }
}

/// Writes the buffer 'pixels', whose dimensions are 
/// specified by the 'bounds' argument, to the file named 'filename'
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize))-> Result<(), std::io::Error> {

    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    //let color = [0x85, 0xC7, 0xAB];
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
    
    Ok(())
}