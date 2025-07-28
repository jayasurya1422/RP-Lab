trait Animal {
    fn make_sound(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}

fn animal_sounds<T: Animal>(animal: T) {
    animal.make_sound();
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    animal_sounds(dog);
    animal_sounds(cat);
}
