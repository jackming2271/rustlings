// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!



fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (age, name) = cat;

    println!("{} is {} years old.", name, age);
}
