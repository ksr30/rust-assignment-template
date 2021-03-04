pub struct Score {
    pub hindi: i32,
    pub english: i32,
    pub maths: i32,
    pub science: i32,
}

pub struct Student {
    pub name: String,
    pub roll_no: i32,
    pub score_of_each_subject: Score,
    pub department: String,
    pub school: String,
}
impl Default for Student {
    fn default() -> Self {
        Student::new()
    }
}

impl Student {
    /// This method initializes Student object.
    ///
    /// #Arguments
    ///
    /// no arguments
    ///
    /// #Return
    ///
    /// Return the Student type object.
    ///
    pub fn new() -> Student {
        Student {
            name: "".parse().unwrap(),
            roll_no: 0,
            score_of_each_subject: Score {
                hindi: 0,
                english: 0,
                maths: 0,
                science: 0,
            },
            department: "".parse().unwrap(),
            school: "".parse().unwrap(),
        }
    }

    /// This function gets average of all marks of a student
    ///
    /// #Arguments
    ///
    /// no arguments
    ///
    /// #Return
    ///
    /// Return the f32 value as average.
    pub fn get_average(&self) -> f32 {
        ((self.score_of_each_subject.english
            + self.score_of_each_subject.hindi
            + self.score_of_each_subject.maths
            + self.score_of_each_subject.science)
            / 4) as f32
    }
    /// This method pass the student if marks are less than 35 in any subject.
    ///
    /// #Arguments
    ///
    /// no arguments
    ///
    /// #Return
    ///
    /// no return type.
    pub fn pass_student(&mut self) {
        if self.score_of_each_subject.science < 35 {
            self.score_of_each_subject.science += 35 - self.score_of_each_subject.science;
        }
        if self.score_of_each_subject.maths < 35 {
            self.score_of_each_subject.maths += 35 - self.score_of_each_subject.maths;
        }
        if self.score_of_each_subject.english < 35 {
            self.score_of_each_subject.english += 35 - self.score_of_each_subject.english;
        }
        if self.score_of_each_subject.hindi < 35 {
            self.score_of_each_subject.hindi += 35 - self.score_of_each_subject.hindi;
        }
    }
    /// This method prints the difference of marks of two students.
    ///
    /// #Arguments
    ///
    /// student type object
    ///
    /// #Return
    ///
    /// no return type.
    pub fn compare_student(&self, stud_to_compare: Student) {
        print!(
            "{} {} {} {}",
            (stud_to_compare.score_of_each_subject.science - self.score_of_each_subject.science)
                .abs(),
            (stud_to_compare.score_of_each_subject.english - self.score_of_each_subject.english)
                .abs(),
            (stud_to_compare.score_of_each_subject.hindi - self.score_of_each_subject.hindi).abs(),
            (stud_to_compare.score_of_each_subject.maths - self.score_of_each_subject.maths).abs()
        )
    }
}
