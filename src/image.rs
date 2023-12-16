use std::fmt::Display;

// implement the Pixel struct and traits below
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r, self.b, self.g)
    }
}
// implement the Image struct and traits below
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Pixel>,
}

impl Image {
    // Allocates a vector that holds width*height pixels and ini2alizes it so that each pixel value is set to Pixel{r: 0, g: 0, b: 0}.
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            data: vec![Pixel { r: 0, g: 0, b: 0 }; width * height],
        }
    }

    // Returns an immutable reference to a Pixel wrapped in an Option enum, which contains some data
    // if the pixel exists, or it returns None if the given x and y values are out of range.
    pub fn get(&self, x: usize, y: usize) -> Option<&Pixel> {
        self.data.get(x + y * self.width)
    }

    // Same as above, excepts it returns a mutable reference.
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        self.data.get_mut(x + y * self.width)
    }

    // Returns the number of pixels in the Mandelbrot set by itera2ng over the data vector using iterators and closures.
    pub fn get_mandelbrot_pixels(&self) -> usize {
        self.data
            .iter()
            .filter(|&&p| p.r == 0 && p.b == 0 && p.g == 0)
            .count()
    }
}
