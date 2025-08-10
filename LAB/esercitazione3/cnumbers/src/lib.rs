pub mod solution;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ComplexNumber {
    pub real: f64,
    pub imag: f64,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ComplexNumberError{
    ImaginaryNotZero,
}