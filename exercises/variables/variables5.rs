// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // can do this beacause number is no longer available, the ownership was given to println!
    println!("Number plus two is : {}", number + 2);
}
