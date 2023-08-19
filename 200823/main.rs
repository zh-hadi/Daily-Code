#![deny(clippy::all)]


fn main(){
    // {
                // rust in stirng move value || threr for dont work this
    //     let name1 = "hadi".to_string();
    //     let name2 = name1; // her name1 is delete in memory so i can not use any more name1 variable . it's call borrow
        
    //     println!("hello {}", name1); 
    //     println!("hello {}", name2);
    // }
    // {
                // same thing working fine in int varible 
    //     let a = 20;
    //     let b = a;
    //     println!("he is {} years old", a);
    //     println!("he is {} years old", b);

    // }

    // ^= its call rust heap 

    // sring hold both heap and stack
    // stirng store in stack three pis es data sotre 
     // 1.prt
     // 2.len
     // 3.capacity
     // heap store 

     // we reference data 
     let s1 = "hadi".to_string();
     let s2 = &s1; // use refernce to value move
     println!("hello {}", s1);
     println!("hello {}", s2);
        // what happend this code after main function     value are deleted s1, s2 refer *ptr point get to read data

     // work in function 
}