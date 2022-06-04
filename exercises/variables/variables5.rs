// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);

    // Shadow
    // https://doc.rust-lang.org/rust-by-example/variable_bindings/scope.html
    //  l' ombrage variable :
    {
        let number = 3;
        println!("Number plus two is : {}", number + 2);
    }
}
