#[derive(Debug)]

///Score Structure
pub struct Score {
    pub hindi: i32,
    pub english: i32,
    pub maths: i32,
    pub science: i32,
}

///Student Structure
pub struct Student {
    name: String,
    roll_no: i32,
    score_of_each_subject: Score,
    department: String,
    school: String,
}

///Implementing Student Structure
impl Student {
    ///This Function is to used initialize Student objects
    pub fn new(other: &Score) -> Student {
        Student {
            name: String::from("Ankit"),
            roll_no: 12,
            score_of_each_subject: Score {
                hindi: other.hindi,
                english: other.english,
                maths: other.maths,
                science: other.science,
            },
            department: String::from("Rust"),
            school: String::from("Mira Model School"),
        }
    }
}

///Implementing Score Structure
impl Score {
    ///This Function check the marks of Student in each subjects
    ///if marks are less than 35 in subject
    ///then add the difference to actual marks
    ///to make student pass the particular subject
    pub fn pass_student(&mut self) -> (i32, i32, i32, i32) {
        let mut result: (i32, i32, i32, i32) = (0, 0, 0, 0);

        if self.hindi < 35 {
            let dif: i32 = 35 - self.hindi;
            self.hindi += dif;
            result.0 = dif;
        }

        if self.english < 35 {
            let dif: i32 = 35 - self.english;
            self.english += dif;
            result.1 = dif;
        }

        if self.maths < 35 {
            let dif: i32 = 35 - self.maths;
            self.maths += dif;
            result.2 = dif;
        }

        if self.science < 35 {
            let dif: i32 = 35 - self.science;
            self.science += dif;
            result.3 = dif;
        }

        result
    }

    ///This Function find the average marks of Student
    pub fn avg_marks(&self) -> i32 {
        self.hindi + self.maths + self.english + self.science
    }

    ///This Function compare the marks of two students
    pub fn compare_student(&self, other: &Score) -> String {
        let total_stu1: i32 = self.hindi + self.maths + self.english + self.science;

        let total_stu2: i32 = other.hindi + other.maths + other.english + other.science;
        let mut result: String = String::new();

        if total_stu1 < total_stu2 {
            result.push_str("Student 2 has more marks then Student 1");
        }

        if total_stu1 == total_stu2 {
            result.push_str("Same Marks of both the Students");
        }

        if total_stu2 < total_stu1 {
            result.push_str("Student 1 has more marks then Student 2");
        }
        result
    }
}