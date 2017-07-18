mod parts;
pub use self::parts::*;
/*
pub struct FormattedCollision {
    points: Vec<CollisionPoint>,
    collision: Vec<CollisionSet>,
    spawns: Vec<Spawn>
}

struct CollisionSet {
    top: Vec<Planes>,
    bottom: Vec<Planes>,
    right: Vec<Planes>,
    left: Vec<Planes>
}

struct Planes(Vec<Vec<usize>>);

impl FormattedCollision {
    fn from_parts(p: Vec<CollisionPoint>, s: Vec<Spawn>,
        pi: &[PlaneInfo], con: &[u16], dirs: &[ColDetection])
    -> Self {
        // The CollisionPoint vec and Spawn vec are directly put into the output?
    }
}*/
