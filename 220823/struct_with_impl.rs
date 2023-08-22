#![deny(clippy::all)]

#[derive(Debug)]

struct Point(f64, f64, f64);

// implement function to this Point struct
impl Point {
    fn display(&self){
        println!("Point at x = {} y = {} z = {}", self.0, self.1, self.2);
    }
    fn make_twice(&mut self){ //mutable method or function
        self.0 *= 2.0;
        self.1 *= 2.0;
        self.2 *= 2.0;
    }
    fn zero()-> Point{ // create non asociated function
        Point(0.0, 0.0, 0.0)
    }
}



fn main(){
    let p =Point (1.0, 2.0, 3.0);
    p.display();

    let mut p2 = Point(10.0, 20.0, 30.0);
    p2.make_twice();
    p2.display();

    let p3 = Point::zero();  // non-asociated function
    p3.display();
}
