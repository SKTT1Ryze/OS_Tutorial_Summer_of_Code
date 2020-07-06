/*
 * Learn Rust The Hard Way: Switch Statement
 * 2020/7/6
 * hustccc
 * Manjaro
 */
#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
    height: i32,
    weight: i32
}

impl Person {
    fn new (name: String, age: i32, height: i32, weight: i32) -> Person{
        Person {
            name: name,
            age: age,
            height: height,
            weight: weight
        }
    }
    fn person_print (&self) {
        println!("{:?}", self);
    }
    fn set_valute (&mut self, name: String, age: i32, height: i32, weight: i32) {
        self.name = name;
        self.age = age;
        self.height = height;
        self.weight = weight;
    }
}
fn main () {
    let mut joe = Person::new("Joe Alex".to_string(),32,64,140);
    let mut frank = Person::new("Frank Blank".to_string(),20,72,180);
    println!("Joe:");
    joe.person_print();
    println!("Frank:");
    frank.person_print();
    joe.set_valute("Joe Alex".to_string(),52,62,180);
    frank.set_valute("Frank Blank".to_string(),40,92,180);
    println!("Joe:");
    joe.person_print();
    println!("Frank:");
    frank.person_print();
}