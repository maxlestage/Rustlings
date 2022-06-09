// move_semantics6.rs
// Make me compile! `rustlings hint move_semantics6` for hints
// You can't change anything except adding or removing references

fn main() {
    // let data = "Rust is great!".to_string();
    // Je rends la variable mutable :
    let mut data = "Rust is great!".to_string();

    // get_char(data);
    // Pour modifier la variable depuis la référance, je dois ajouter une référence à data --> &data
    // et je dois le faire sur le type attendu de la function fn get_char(data: &String) -> char {..
    get_char(&data);

    // string_uppercase(&data);
    // data est mutable donc on l'envoie vers la fonction fn string_uppercase(mut data: String) {.. directment
    string_uppercase(data);
}

// Should not take ownership
// fn get_char(data: String) -> char {
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
// fn string_uppercase(mut data: String) {
// ici on rappel que la valeur d'entrée de data est mutable et on peut alors modifier sa valeur.
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}

/*

Avant :
fn main() {
    let data = "Rust is great!".to_string();

    get_char(data);

    string_uppercase(&data);
}

/// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

/// Should take ownership
fn string_uppercase(mut data: &String) {
    data = &data.to_uppercase();

    println!("{}", data);
}


*/
