 
#![deny(clippy::all)]

#[derive(PartialEq)]
#[derive(Debug)]
enum Home {
    own(String),
    rent,
}

struct Person {
    name : String,
    id : u32,
}

enum Shape {
    Rectangel{width: f64, height: f64},
}

fn main(){
    let home = Home::own(String::from("hello home"));
    println!("{:?}", home);

    let person = Person{name: "hadi".to_string(), id: 32};
    println!(" {} {} ", person.name, person.id);


    let rectangel = Shape::Rectangel { width: 20.0, height: 40.0};

    let Shape::Rectangel { width, height } = rectangel;
    // how to i use this enum and print
    println!("width = {}  height = {}", width, height);

}
