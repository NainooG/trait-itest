pub trait Test {
    fn test_trait(self);
    fn lying(self);
    fn fart(self);
}

struct Person {
    name: String,
    age: i32,
}

impl Test for Person {
    fn test_trait(self) {
        println!("testing trait {} {}!", self.name, self.age);
    }

    fn lying(self) {
        println!("{} is lying!", self.name);
    }

    fn fart(self) {
        println!("{} farted", self.name);
    }
}

// name: String::from("andybobandy"),

fn main() {
    let person = Person {
        name: "andybobandy".to_string(),
        age: 25,
    };

    // person.test_trait();
    // person.lying();
    person.fart();
}
