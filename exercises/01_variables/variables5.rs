fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number: &str = "T-H-R-E-E";
    println!("Number plus two is: {}", number.to_owned() + "2");
}
