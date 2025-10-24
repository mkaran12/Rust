fn main() {
    let pi: f64 = 3.14159;
    println!("The value of pi is: {}", pi);

    println!("{}", pi.floor());
    println!("{}", pi.ceil());
    println!("{}", pi.round());
    println!("{}", pi.trunc());
    println!("{}", pi.fract());
    //format specifer customziess the printed representation of an intoraplated value
    println!("{:.2}", pi); //prints pi value with 2 decimal places


}
