#![deny(clippy::all)]


fn info()->(String, String, i32){
    ("Hadiuzzaman".to_string(), "Dhaka".to_string(), 4500)
}


fn main(){
    // touple example
    let tp = (10, 20, 30); // create touple
    let (a, b, c) = tp; // distruct touple
    println!(" a = {} b = {} c = {}", a, b, c);

    let tp2 = ("hadi", "jessore", 7400);
    let (name, address, pcode) = tp2;
    println!("Name = {}\nAddress = {}\nPost code = {}", name, address, pcode);

    let (m, n, p) = info();
    println!("{} {} {}", m, n, p);

    // now learn vector
    //let ar:[i32; 4] = [1,2,3,4];
    //use vector
    let ar = vec![10, 20, 30, 40, 50, 60];
    println!("{} {} {} {}", ar[0], ar[1], ar[2], ar[3]);

    let length = ar.len();
    let mut i = 0;
    loop {
        println!("index[{}] = {}", i, ar[i]);
        i+=1;
        if i >= length{
            break;
        }
    }

    println!("=========\n");
    for values in ar.iter(){
        println!("{}", values);
    }

}
