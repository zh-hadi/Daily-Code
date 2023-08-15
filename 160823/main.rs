#![deny(clippy::all)]

const MY_NAME: &str = "Hadiuzzaman";
fn main() {
    let mut age:i32 = 20;
    age = 34;

    let num = 2.0f32;
    let num2 = 3.0;
    println!("{}", num + num2);
    //let age: = 30;

    println!("{}", age);
    println!("my name is {}", MY_NAME);

    // learn toupel
    let person = (21, "Hadiuzzaman");
    let (age, name) = person;
    println!("age = {} name = {}", person.0, person.1);
    println!("{} {}", age, name);

    let salary = 20_000;
    println!("salary : {}", salary);


    // who to write variable
    // rust environment setup
    // use clippy 
    // cargo-watch pkg
    // cargo-watch -qc -x run -x clippy
    // opation == left to write
    // mutable and unmuteable 
    // variable shadow
    // tuple 
    // tuple unpack
    // float , &str , int
    // formate specifier for output data

}
