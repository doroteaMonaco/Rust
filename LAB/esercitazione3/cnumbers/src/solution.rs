use std::ops::Add;
use std::fmt::Display;
use std::ops::AddAssign;
use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ComplexNumber {
    pub real: f64,
    pub imag: f64,
}

impl ComplexNumber {
    pub fn new (real : f64, imag : f64) -> Self{
        ComplexNumber {real, imag}
    }
    pub fn from_real(real : f64) -> Self{
        ComplexNumber {real, imag: 0.0}
    }
    pub fn real(&self) -> f64{
        self.real
    }
    pub fn imag(&self) -> f64{
        self.imag
    }
    pub fn to_tuple(&self) -> (f64, f64){
        (self.real, self.imag)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ComplexNumberError{
    ImaginaryNotZero,
}

impl Add for ComplexNumber{
    type Output = Self;

    fn add(self, other: Self) -> Self{
        ComplexNumber {real: self.real + other.real, imag: self.imag + other.imag}
    }
}

impl Add<f64> for ComplexNumber{
    type Output = Self;

    fn add(self, other: f64) -> Self{
        ComplexNumber {real: self.real + other, imag: self.imag}
    }
}

impl Add<&ComplexNumber> for ComplexNumber{
    type Output = Self;

    fn add(self, other: &ComplexNumber) -> Self{
        ComplexNumber {real: self.real + other.real, imag: self.imag + other.imag}
    }
}

impl Add<&ComplexNumber> for &ComplexNumber{
    type Output = ComplexNumber;

    fn add(self, other: &ComplexNumber) -> ComplexNumber{
        ComplexNumber {real: self.real + other.real, imag: self.imag + other.imag}
    }
}

impl AddAssign for ComplexNumber{
    fn add_assign(&mut self, other: Self){
        self.real += other.real;
        self.imag += other.imag;
    }
}

impl Display for ComplexNumber{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.imag >= 0.0 {
            write!(f, "{} + {}i", self.real, self.imag)
        }
        else {
            write!(f, "{} - {}i", self.real, -self.imag)
        }
    }
}

impl Default for ComplexNumber{
    fn default() -> Self {
        ComplexNumber {real: 0.0, imag: 0.0}
    }
}


impl Into<f64> for ComplexNumber{
    fn into(self) -> f64 {
        if self.imag == 0.0 {
            self.real
        } else {
            panic!("Cannot convert complex number to real number")
        }
    }
}

impl Into<ComplexNumber> for f64{
    fn into(self) -> ComplexNumber {
        ComplexNumber {real: self, imag: 0.0}
    }
}


/* 
impl TryInto<f64> for ComplexNumber{
    type Error = ComplexNumberError;

    fn try_into(self) -> Result<f64, Self::Error> {
        if self.imag == 0.0 {
            Ok(self.real)
        } else {
            Err(ComplexNumberError::ImaginaryNotZero)
        }
    }
}
*/

impl Ord for ComplexNumber {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.real == other.real {
            if self.imag == other.imag {
                std::cmp::Ordering::Equal
            } else if self.imag < other.imag {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        } else if self.real < other.real {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

impl Eq for ComplexNumber {
}

impl PartialOrd for ComplexNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl AsRef<f64> for ComplexNumber {
    fn as_ref(&self) -> &f64 {
        &self.real
    }
}

impl AsMut<f64> for ComplexNumber {
    fn as_mut(&mut self) -> &mut f64 {
        &mut self.real
    }
}

impl Hash for ComplexNumber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.real.to_bits().hash(state);
        self.imag.to_bits().hash(state);
    }
}




