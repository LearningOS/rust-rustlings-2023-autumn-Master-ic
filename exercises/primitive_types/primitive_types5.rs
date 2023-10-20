// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.


fn main(){
    let cat =("Furry McFurson",3.5);
    let (name,age) = cat ;
    println!("{} is {} years old.",name ,age);
}
