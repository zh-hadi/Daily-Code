#![deny(clippy::all)]


fn main(){
    // // rust code exicuted express way
    let a = "hadi";
    println!(" {a}");

    let a = {
        let b = 20;
        b + 10 // is expression for return
    };
    println!("{a}");

        // --------------------- loop example -------------
    let mut count = 10;
    loop {
        println!("{count}");
        if count == 0 {
            break;
        }
        count -=1;
    }

        // --------------------- array example --------------------
    let ar = [10, 20, 30, 40, 50, 60, 70, 80];
    let mut i =0;
    while i<9 {

        println!("index = {}  value = {}", i, ar[i]);
        i+=1;
    }

}


