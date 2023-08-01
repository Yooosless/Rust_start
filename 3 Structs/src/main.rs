#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

struct Rectangle2{
    width: f32,
    height:f32,
}
//task1
fn rect_area(rectangle: &Rectangle) -> f32 {
    let Rectangle { top_left, bottom_right } = rectangle;
    let width = bottom_right.x - top_left.x;
    let height = top_left.y - bottom_right.y;
    width * height
}
//basic one with direct width and height
fn rectangle_area(rectange: &Rectangle2)->f32{
    let Rectangle2{width,height}=rectange;
    width*height
}


fn main() {

    let rect = Rectangle {
        top_left: Point { x: 1.0, y: 5.0 },
        bottom_right: Point { x: 4.0, y: 2.0 },
    };
    let area = rect_area(&rect);
    println!("Area of the rectangle: {}", area);
    let rectan=Rectangle2{
        width:20.0,
        height:22.0,
    };
    let area2=rectangle_area(&rectan);
    println!("Area of second rectangle is: {}", area2);
}

