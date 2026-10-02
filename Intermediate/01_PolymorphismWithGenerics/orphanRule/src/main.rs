use orphanRule::Point;

struct PointWrapper(Point);

// impl PartialEq for Point
impl PartialEq for PointWrapper {
    // This will not worked because PartialEq trait is implemented in Rust main documentation or library and Point struct is implemented in this local codebase ....and this situation called the orphan rule... to solve this issues, we need to create an struct containing tuple.
    fn eq(&self, other: &Self) -> bool {
        // self.x == other.x && self.y == other.y
        self.0.x == other.0.x && self.0.y == other.0.y // accessing tuple value with index 0. 
    }
}

fn main() {
    let p1 = PointWrapper(Point { x: 1, y: 2 });
    let p2 = PointWrapper(Point { x: 1, y: 2 });

    println!("{}", p1 == p2); // This comparision is possible because of `PartialEq` trait. 
}
