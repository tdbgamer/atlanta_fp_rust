use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

fn main() {
    let first = &Point { x: 1, y: 2 };
    let second = &Point { x: 2, y: 3 };
    dbg!(first);
    dbg!(second);
    dbg!(first + second);
}
