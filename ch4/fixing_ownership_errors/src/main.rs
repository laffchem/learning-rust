fn main() {
    let mut name = vec![String::from("Ferris"), String::from("Bueller")];
    let full_name = stringify_name_with_title(&mut name);
    println!("Full name: {}", full_name);
}

// will fail
// fn return_a_string() -> &String {
//     let s = String::from("Hello world");
//     &s
// }
// one of many options
// fn return_a_string() -> String {
//     let s = String::from("Hello world");
//     s
// }
// Rejects because &Vec is read only, need a mutable reference
// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }
// Best way to do this without cloning variables.
fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}