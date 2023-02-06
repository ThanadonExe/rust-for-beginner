#[derive(Debug)]
struct Student{
	name: String,
	math: i32,
	science: i32,
	art: i32,
}

impl Student{
    fn build(name:String, math:i32, science:i32, art:i32) -> Student {
		Student{name, math, science, art}
	}

	fn highest(&self) {
		if self.math > self.science && self.math > self.art {
			println!("{} got highest marks in Maths", self.name);
		} else if self.science > self.math && self.science > self.art {
			println!("{} got highest marks in Sciences", self.name);
		} else {
			println!("{} got highest marks in Arts", self.name);
		}
	}
}


fn main() {
	let s1 = Student::build(String::from("Adam"), 90, 95, 99);
	s1.highest();
}
