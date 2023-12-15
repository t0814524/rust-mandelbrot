// implement the Pixel struct and traits below
#[derive(Debug, Copy, Clone, PartialEq, Display)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Display for Pixel{
    fn
}

// implement the Image struct and traits below
struct Image {
    width: usize,
    height: usize,
    data:  Vec<Pixel>,
}

// Allocates a vector that holds width*height pixels and ini2alizes it so that each pixel value is set to Pixel{r: 0, g: 0, b: 0}.
pub fn new(width: usize, height: usize ) -> Image

// Returns an immutable reference to a Pixel wrapped in an Option enum, which contains some data 
// if the pixel exists, or it returns None if the given x and y values are out of range.
pub fn get(&self, x: usize, y: usize) -> Option<&Pixel>

// Same as above, excepts it returns a mutable reference.
pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel>

// Returns the number of pixels in the Mandelbrot set by itera2ng over the data vector using iterators and closures.
pub fn get_mandelbrot_pixels(&self) -> usize