#![deny(clippy::all)]
// learn struct

struct Person {
    name: String,
    age: u8,
    id: i32,
    add: String,
}

// struct function to write data
// fn struct_fn(name: String, age:u8)-> Person{
//     Person{
//         name,
//         age,
//     }
// }

// how to write touple in struct
struct Point (f64, f64, f64);

fn main(){
    //let person = Person{"Hadiuzzaman".to_string(), 23};
    let person = Person{
        name: "hadiuzzaman".to_string(),
        age: 23,
        id: 2023,
        add : "jessore".to_string(),
    };

    println!("{} is {} years old id=  {} address=  {}", person.name, person.age, person.id, person.add);

    // struct init initialization
    // let person1 = Person{ String::from("hadiuzzaman"), 23};
    // println!("{} is {} years old", person1.name, person1.age);
    //let name = "Hadiuzzaman".to_string();
    //let p1 = struct_fn(name, 56);
    //println!("{} is {} years old", p1.name, p1.age);

    let person2 = Person{
        name: "babor".to_string(),
        ..person
    };
    println!("{} is {} years old id=  {} address=  {}", person2.name, person2.age, person2.id, person2.add);
    // struct data change other struct

}
