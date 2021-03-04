#[derive(Debug)]
pub struct Numbers {
    pub real_part: i32,
    pub image_part: i32,
}

impl Numbers {
    ///This Function add two complex numbers
    pub fn add_complex_numbers(&self, other: &Numbers) -> (i32, i32) {
        let add_real: i32 = self.real_part + other.real_part;
        let add_image: i32 = self.image_part + other.image_part;
        let result: (i32, i32) = (add_real, add_image);
        result
    }

    ///This Function find the difference between two complex numbers
    pub fn sub_complex_numbers(&self, other: &Numbers) -> (i32, i32) {
        let sub_real: i32 = self.real_part - other.real_part;
        let sub_image: i32 = self.image_part - other.image_part;
        let result: (i32, i32) = (sub_real, sub_image);
        result
    }

    ///This Function multiply two complex numbers
    pub fn mul_complex_numbers(&self, other: &Numbers) -> (i32, i32) {
        let mul_real: i32 = self.real_part * other.real_part - self.image_part * other.image_part;
        let mul_image: i32 = self.real_part * other.image_part - self.image_part * other.real_part;
        let result: (i32, i32) = (mul_real, mul_image);
        result
    }
}