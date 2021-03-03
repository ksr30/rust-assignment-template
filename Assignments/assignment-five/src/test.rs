#[cfg(test)]
mod tests {
    use crate::complex::ComplexNumber;
    use crate::student_score::Student;
    #[test]
    fn adding_complex_number() {
        let complex1 = ComplexNumber { real: 10, imag: 20 };
        let complex2 = ComplexNumber { real: 20, imag: 40 };
        let output = complex1.add(complex2);
        assert_eq!(output.real, 30);
        assert_eq!(output.imag, 60);
    }
    #[test]
    fn subtract_complex_number() {
        let complex1 = ComplexNumber { real: 10, imag: 20 };
        let complex2 = ComplexNumber { real: 20, imag: 40 };
        let output = complex1.subtract(complex2);
        assert_eq!(output.real, -10);
        assert_eq!(output.imag, -20);
    }
    #[test]
    fn multiply_complex_number() {
        let complex1 = ComplexNumber { real: 0, imag: 200 };
        let complex2 = ComplexNumber {
            real: 210,
            imag: 400,
        };
        let output = complex1.multiply(complex2);
        assert_eq!(output.real, 0);
        assert_eq!(output.imag, 80000);
    }
    #[test]
    fn checking_new_in_student() {
        let testobj = Student::new();
        assert_eq!(testobj.name, "");
        assert_eq!(testobj.roll_no, 0);
        assert_eq!(testobj.department, "");
        assert_eq!(testobj.school, "");
        assert_eq!(testobj.score_of_each_subject.english, 0);
        assert_eq!(testobj.score_of_each_subject.hindi, 0);
        assert_eq!(testobj.score_of_each_subject.maths, 0);
        assert_eq!(testobj.score_of_each_subject.science, 0);
    }
    #[test]
    fn get_average() {
        let mut testobj = Student::new();
        testobj.score_of_each_subject.science = 10;
        testobj.score_of_each_subject.maths = 20;
        testobj.score_of_each_subject.english = 30;
        testobj.score_of_each_subject.hindi = 40;
        assert_eq!(testobj.get_average(), 25 as f32);
    }
    #[test]
    fn pass_student() {
        let mut testobj = Student::new();
        testobj.score_of_each_subject.science = 70;
        testobj.score_of_each_subject.maths = 20;
        testobj.score_of_each_subject.english = 50;
        testobj.score_of_each_subject.hindi = 10;
        testobj.pass_student();
        assert_eq!(testobj.score_of_each_subject.hindi, 35);
        assert_eq!(testobj.score_of_each_subject.english, 50);
        assert_eq!(testobj.score_of_each_subject.maths, 35);
        assert_eq!(testobj.score_of_each_subject.science, 70);
    }
}
