// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

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

    // Je passe la variable a ?? la function :
    push_in_array(&mut a);

    // Condition qui v??rifie si a est supp' ?? 100 ou non :
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}

// Function qui me permet d'ajouter des ??l??ments dans la variable a :
fn push_in_array(a: &mut Vec<&str>) {
    // D??fini un index i qui vaut 0 :
    let mut i: i32 = 0;

    // Tant que i vaut moins de 100 :
    while i < 100 {
        // je push "test" dans a :
        &a.push("test");
        // j'affiche la longueur du vecteur a ?? ce moment :
        println!("{}", a.len());

        // J'incr??mente i + 1 :
        i += 1;
    }
}
