// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

// I AM NOT DONE
/*
fn main() {
    let a = [
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
        "edzdopf",
        "eded",
        "ededed",
        "edzedzdezz",
    ];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
 */

fn main() {
    // Init le Vec :
    let mut a: Vec<&str> = vec![];

    // Je passe la variable a à la function :
    push_in_array(&mut a);

    // Condition qui verifie si a est supp à 100 ou non :
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}

// Function qui me permet d'ajouter des éléments dans la variable a :
fn push_in_array(a: &mut Vec<&str>) {
    // Défini un index i qui vaut 0 :
    let mut i: i32 = 0;

    // Tant que i vaut moins de 100 :
    while i < 100 {
        // je push "test" dans a :
        &a.push("test");
        // j'affiche la longeure du vecteur a à ce moment :
        println!("{}", a.len());

        // J'incrémente i + 1 :
        i += 1;
    }
}
