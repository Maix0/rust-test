fn main() {
    let rect = Rectangle {p1:Point {x:9.5,y:1.63}, p2:Point {x:10.0,y:10.0}};
    let rectDebug = dRectangle {p1: dPoint {x:10.0,y:5.0},p2: dPoint {x:9.0,y:7.0}};

    println!("{}", rect);
    println!("");
    println!("{:#?}",rectDebug)
}


struct Point {
    x:f64,
    y:f64,
}
struct Rectangle {
    p1:Point,
    p2:Point,
}


impl std::fmt::Display for Rectangle {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "Rectangle {{\n    p1: {},\n    p2: {} \n}}", self.p1 , self.p2); //  /**self.p1, *self.p2**/));
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "Point {{\n    x:{},\n    y: {}\n}}", self.x,self.y)
    }   

}
#[derive(Debug)]
struct dPoint {
    x:f64,
    y:f64,
}

#[derive(Debug)]
struct dRectangle {
    p1:dPoint,
    p2:dPoint
}


