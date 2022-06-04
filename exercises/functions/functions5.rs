// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
    // implicitly returns `()` as its body has no tail or `return` expression
    /*
    Pour renvoyer une valeur à partir d'une fonction, vous avez deux options dans rust :

    Utiliser une return <some value>;expression
    Terminer la fonction par une expression ou une valeur sans point-virgule
    // returns the value '1' of type 'u8' without using a 'return'
    fn return_one() -> u8 {
        1
    }
    Lorsqu'aucune de ces options n'est utilisée, c'est-à-dire que le bloc fonctionnel se termine par un point-virgule comme le fait votre fonction, la valeur renvoyée est ()également appelée type d'unité. Cette valeur est également renvoyée par les fonctions qui ne déclarent pas un type de retour comme celui-ci :

    // This function returns the unit type '()',
    // because it ends with a semicolon and has no return expressions.
    fn print_to_console(message: &str) {
        println!(message);
    } */
}
