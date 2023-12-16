use std::f64;

use crate::complex::Complex;
use crate::image::Image;
pub fn check_pixel(c: Complex, max_iterations: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let mut i = 0;
    while i < max_iterations {
        z = z * z + c;
        if z.mag() > 4.0 {
            return Some(i); // not in mandelbrot set
        };
        i += 1;
    }
    None // magnitude <= 4 means it is in the mandelbrot set
}

/**
 * > quote: Note that you need to use the Image struct and implemented traits to access the values in associated vector (e.g., image.get(x, y) or image.get_mut(x, y)).
 *      definitely dont need to, makes no sense as i already have the a valid idx/
 *
 * > quote: Note that this function calls the check_pixel function, which checks if the pixel belongs to the Mandelbrot set.
 * If the check_pixel function returns "Some" value, we set the color of that pixel to white, otherwise we set it to black (since it means it is in the Mandelbrot set).
 *      
*/
pub fn generate_image(width: usize, height: usize, max_iterations: usize) -> Image {
    let mut image = Image::new(width, height);
    for i in 0..image.data.len() {
        let y = (i as f64 / width as f64).floor() as usize;
        let x = i - y * width;
        // println!("y:{}, x:{}", y, x);
        let cx = (x as f64 / image.width as f64 - 0.75) * 3.5;
        let cy = (y as f64 / image.height as f64 - 0.5) * 2.0;
        let c = Complex { re: cx, im: cy };

        match check_pixel(c, max_iterations) {
            Some(_) => {} // alredy initialized values with 0,0,0
            None => {
                // let p = image.data[i];
                // idk what for but says so in specification
                match image.get_mut(x, y) {
                    Some(p) => {
                        p.r = 255;
                        p.b = 255;
                        p.g = 255;
                    }
                    None => {
                        println!("wtf");
                    }
                }
            }
        }
    }
    image
}
