fn main() {
    // quiz::checkbox("Select your colors", &["red", "green", "blue"]);
    let res = quiz::confirm("Do you agree?");
    println!("{}", res);
}
