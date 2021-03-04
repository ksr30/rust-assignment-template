pub struct ComplexNumber {
    pub real: i32,
    pub imag: i32,
}

impl ComplexNumber {
    /// This method adds two complex numbers.
    ///
    /// #Arguments
    ///
    /// complex type object
    ///
    /// #Return
    ///
    /// Returns the complex type object
    pub fn add(&self, second_no: ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            real: self.real + second_no.real,
            imag: self.imag + second_no.imag,
        }
    }
    /// This method subtracts two complex numbers.
    ///
    /// #Arguments
    ///
    /// complex type object
    ///
    /// #Return
    ///
    /// Returns the complex type object
    pub fn subtract(&self, second_no: ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            real: self.real - second_no.real,
            imag: self.imag - second_no.imag,
        }
    }
    /// This method multiply two complex numbers.
    ///
    /// #Arguments
    ///
    /// complex type object
    ///
    /// #Return
    ///
    /// Returns the complex type object
    pub fn multiply(&self, second_no: ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            real: self.real * second_no.real,
            imag: self.imag * second_no.imag,
        }
    }
}
