#[cfg(test)]

mod tests {
    use crate::ques2::Score;
    use crate::ques1::Numbers;

    #[test]
    fn check_add() {
        let num1 :Numbers = Numbers {
            real_part: 15,
            image_part: 6,
        };
        let num2 :Numbers = Numbers {
            real_part: 10,
            image_part: 10,
        };
        assert_eq!(num1.add_complex_numbers(&num2), (25, 16));
    }

    #[test]
    fn check_sub() {
        let num1 :Numbers= Numbers {
            real_part: 15,
            image_part: 6,
        };
        let num2 :Numbers = Numbers {
            real_part: 10,
            image_part: 10,
        };
        assert_eq!(num1.sub_complex_numbers(&num2), (5, -4));
    }

    #[test]
    fn check_mul() {
        let num1 :Numbers = Numbers {
            real_part: 15,
            image_part: 6,
        };
        let num2 :Numbers = Numbers {
            real_part: 10,
            image_part: 10,
        };
        let mul: (i32, i32) =  num1.mul_complex_numbers(&num2);
        assert_eq!(mul, (90, 90));
    }

    #[test]
    fn check_pass_student() {
        let mut stu1 :Score= Score {
            hindi: 30,
            english: 25,
            maths: 20,
            science: 31,
        };
        assert_eq!(stu1.pass_student(), (5, 10, 15, 4));
    }

    #[test]
    fn check_pass_student_no() {
        let mut stu1 :Score = Score {
            hindi: 40,
            english: 55,
            maths: 70,
            science: 61,
        };
        assert_eq!(stu1.pass_student(), (0, 0, 0, 0));
    }

    #[test]
    fn check_pass_student_1() {
        let mut stu1:Score = Score {
            hindi: 30,
            english: 55,
            maths: 70,
            science: 21,
        };
        assert_eq!(stu1.pass_student(), (5, 0, 0, 14));
    }

    #[test]
    fn check_pass_student_2() {
        let mut stu1:Score = Score {
            hindi: 90,
            english: 15,
            maths: 10,
            science: 40,
        };
        assert_eq!(stu1.pass_student(), (0, 20, 25, 0));
    }

    #[test]
    fn check_avg_stu1() {
        let stu1:Score = Score {
            hindi: 30,
            english: 25,
            maths: 20,
            science: 31,
        };
        assert_eq!(stu1.avg_marks(), 106);
    }

    #[test]
    fn check_avg_stu2() {
        let stu2 :Score = Score {
            hindi: 50,
            english: 55,
            maths: 70,
            science: 61,
        };
        assert_eq!(stu2.avg_marks(), 236);
    }

    #[test]
    fn check_avg_stu3() {
        let stu3 :Score = Score {
            hindi: 0,
            english: 0,
            maths: 0,
            science: 0,
        };
        assert_eq!(stu3.avg_marks(), 0);
    }

    #[test]
    fn check_compare_student() {
        let stu1 :Score = Score {
            hindi: 30,
            english: 25,
            maths: 20,
            science: 31,
        };
        let stu2 :Score = Score {
            hindi: 60,
            english: 55,
            maths: 50,
            science: 35,
        };
        assert_eq!(
            stu1.compare_student(&stu2),
            "Student 2 has more marks then Student 1"
        );
    }

    #[test]
    fn check_compare_student_1() {
        let stu1 :Score = Score {
            hindi: 100,
            english: 65,
            maths: 70,
            science: 40,
        };
        let stu2  :Score = Score {
            hindi: 60,
            english: 55,
            maths: 50,
            science: 35,
        };
        assert_eq!(
            stu1.compare_student(&stu2),
            "Student 1 has more marks then Student 2"
        );
    }

    #[test]
    fn check_compare_student_same() {
        let stu1 :Score = Score {
            hindi: 30,
            english: 25,
            maths: 20,
            science: 31,
        };
        let stu2 :Score = Score {
            hindi: 30,
            english: 25,
            maths: 20,
            science: 31,
        };
        assert_eq!(
            stu1.compare_student(&stu2),
            "Same Marks of both the Students"
        );
    }
}
