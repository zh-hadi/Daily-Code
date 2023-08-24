#![deny(clippy::all)]


#[derive(Debug)]

struct Student {
    name: i32,
    id: i32
}

fn main(){


  let mut a = vec![10, 20];
  a.push(30);
  a.push(40);
  a.push(50);

  a.extend_from_slice(&[60, 70, 80, 90]);

  let p = a.pop();

    println!("{:?}", a);
    println!("{:?}", p);
    a.clear();
    println!("{:?}", a);

    let s1 = Student{name: 34, id: 35};
    println!("{:?}", s1);


    let mut values1 = vec![1,2,3];
    let mut values2 = vec![4,5,6];
    println!("values1 = {:?}", values1);
    println!("values2 = {:?}", values2);
    values1.append(&mut values2);
    println!("values1 = {:?}", values1);
    println!("values2 = {:?}", values2);

    if values1.contains(&7){
        println!("yes");
    }else {
        println!("no");
    }

    let mut x = vec![10];
    x.clear();
    if x.is_empty() {
        println!(" this is empty vector");
    }

}
