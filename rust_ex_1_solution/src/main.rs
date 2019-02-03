use std::ops::Add;

#[derive(Debug)]
struct Vector {
    x: u32,
    y: u32,
}

impl Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

fn main() {
    let first = &Vector { x: 1, y: 2 };
    let second = &Vector { x: 2, y: 3 };
    dbg!(first);
    dbg!(second);
    dbg!(first + second);
}
