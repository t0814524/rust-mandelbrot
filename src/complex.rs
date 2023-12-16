use std::ops::{Add, Mul};

// implement the Complex struct and traits below
#[derive(Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

//performs mul]plica]on of two complex numbers.
//      Mul2plica2on of 2 complex numbers is defined with:
//      (𝑎 + 𝑏𝑖) ∗ (𝑐 + 𝑑𝑖) = (𝑎𝑐 − 𝑏𝑑) + (𝑎𝑑 + 𝑏𝑐)𝑖
impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Complex {
    // performs addi]on of two Complex numbers.
    //      Addi2on of 2 complex numbers means simply adding the real parts of the number with the complex parts of
    // pub fn Add(&self, other: &Self) -> Complex {
    //     Complex {
    //         re: self.re + other.re,
    //         im: self.im + other.im,
    //     }
    // }

    // calculates the magnitude of the current complex number.
    //      Instead of calcula]ng the actual absolute value (since we are comparing against the value 4 in our
    //      algorithm), we calculate the following:
    //      (𝑎^2 + 𝑏^2)
    pub fn mag(&self) -> f64 {
        self.re.powi(2) + self.im.powi(2)
    }
}
