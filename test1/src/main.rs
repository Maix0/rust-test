
struct Point {
    x:f32,
    y:f32,
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    let point1: Point = Point { x:10.0,y:10.0};
    let point2: Point = Point {x:5.0,y:5.0};
    let rectangle:Rectangle = Rectangle {p1:point1,p2:point2};    
    println!("Point1 : \n x:{:} y:{:} \nPoint2 : \n x: {:} y{:}",rectangle.p1.x,rectangle.p1.y, rectangle.p2.x,rectangle.p2.y);
    println!("{:}",5.0);
}
