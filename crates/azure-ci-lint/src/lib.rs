pub trait Greeter {
    fn greet(&self) -> &str;
}

pub struct Person {
    pub name: String,
}

impl Person {
    pub fn into_greeter(self) -> Box<dyn Greeter> {
        Box::new(self)
    }
}

impl Greeter for Person {
    fn greet(&self) -> &str {
        "Hello"
    }
}
