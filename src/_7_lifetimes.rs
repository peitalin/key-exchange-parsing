

pub fn life() {
    let p1 = Point { x: 5, y: 10.1 };
    let p2 = Point { x: "a", y: "d" };
    let p3 = p1.mixup(p2);
    println!("{} {}", p3.x, p3.y);
}


struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}


