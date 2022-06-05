// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)

fn main() {
    //Rajout des commentaires pour comprendre le cheminement.

    // L'opérateur * permet de pointer vers une valeur ou de la Deref

    let mut x = 100;
    // println!("x vaut : {}", x);

    let y = &mut x;
    // println!("y vaut : {}", y);
    *y += 100;
    // println!("y vaut : {}", y);

    let z = &mut x;
    // println!("z vaut : {}", z);
    *z += 1000;
    // println!("z vaut : {}", z);

    // println!();
    // println!();
    // println!("{}", x);

    assert_eq!(x, 1200);
}
