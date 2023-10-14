trait Animal {
    fn noise(&self);
}

pub struct Sheep {}

impl Animal for Sheep {
    fn noise(&self) {
        println!("mamamama!");
    }
}


fn random_animal(random_number: f64) -> Box<dyn Animal> {
    Box::new(Sheep {})
}