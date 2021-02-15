trait NewTrait: Mammal + std::clone::Clone {}

struct CloningLab<T: NewTrait> {
    subjects: Vec<Box<T>>,
}

impl<T: NewTrait> CloningLab<T> {
    fn clone_subjects(&self) -> Vec<Box<T>> {
        self.subjects.clone()
    }
}

trait Mammal {
    fn walk(&self);
    fn run(&self);
}

#[derive(Clone)]
struct Cat {
    meow_factor: u8,
    purr_factor: u8,
}

impl Mammal for Cat {
    fn walk(&self) {
        println!("Cat::walk");
    }
    fn run(&self) {
        println!("Cat::run")
    }
}

fn main() {}
