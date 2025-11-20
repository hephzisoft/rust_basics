// Struct #2
//
// struct with functions
//
struct Education {
    college_name: String,
    major_title: String,
    minor_title: String,
}

// the impl keyword is primary used to define
// implmentation on types.
//
impl Education {
    fn constructing_education(college: &str, major: &str, minor: &str) -> Self {
        Education {
            college_name: college.to_string(),
            major_title: major.to_string(),
            minor_title: minor.to_string(),
        }
    }

    fn full_education(&self) -> String {
        format!(
            "{}, {}, {}",
            self.college_name, self.major_title, self.minor_title
        )
    }
     // Chnage major
        fn change_major(&mut self, major: &str){
            self.major_title =major.to_string();
        }
    
}

fn main() {
    let mut  education1 = Education::constructing_education("Cambridge", "Maths", "Astrophysics");

    println!(
        "Education: College: {} \nMajor: {} \nMinor: {}",
        education1.college_name, education1.major_title, education1.minor_title
    );

    println!("Education: {}", education1.full_education());

    education1.change_major("Computer Science");

    println!("Education: {}", education1.full_education());
}
