#[derive(Default, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32
}


#[derive(Clone, Debug)]
struct Polyline {
    points: Vec<Point>,
}



fn main() {
    let p1 = Point::default();
    let p2 = Point::default();
    let p3 = Point {x : 10, y :11};
    let pl = Polyline { points : vec![p1,p2,p3]};
    let clone = pl.clone();
    println!("p1:{:?},\n p2:{:?},\n p3:{:?},\n pl:{:?},\n clone:{:?}", p1, p2, p3, pl, clone);
}
