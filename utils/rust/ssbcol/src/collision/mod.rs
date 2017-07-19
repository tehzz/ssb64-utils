mod parts;
pub use self::parts::*;

#[derive(Debug, Serialize)]
pub struct FormattedCollision {
    points: Vec<CollisionPoint>,
    collision: Vec<CollisionSet>,
    spawns: Vec<Spawn>
}
#[derive(Debug, Serialize)]
struct CollisionSet {
    id: u16,
    top: Vec<Planes>,
    bottom: Vec<Planes>,
    right: Vec<Planes>,
    left: Vec<Planes>
}
#[derive(Debug, Serialize)]
struct Planes(Vec<u16>);

impl FormattedCollision {
    pub fn from_parts(p: Vec<CollisionPoint>, s: Vec<Spawn>,
        pi: &[PlaneInfo], connections: &[u16], dirs: &[ColDetection])
    -> Self {
        // The CollisionPoint vec and Spawn vec are directly put into the output?

        // combine plane info, connections to make vec of usize offsets into CollisionPoint vec
        // read each ColDetection struct
        // for each direction, get each plane info
        // for each plane info, read into a vec the connections
        let col_sets = dirs.iter().map(|dir| {
            let id = dir.get_id();
            let (t_s, t_l) = dir.get_top();
            let t_end = (t_s + t_l) as usize;
            let top = condense_to_planes(&pi[t_s as usize..t_end], connections);

            let (b_s, b_l) = dir.get_bottom();
            let b_end = (b_s + b_l) as usize;
            let bottom = condense_to_planes(&pi[b_s as usize..b_end], connections);

            let (r_s, r_l) = dir.get_right();
            let r_end = (r_s + r_l) as usize;
            let right = condense_to_planes(&pi[r_s as usize..r_end], connections);

            let (l_s, l_l) = dir.get_left();
            let l_end = (l_s + l_l) as usize;
            let left = condense_to_planes(&pi[l_s as usize..l_end], connections);

            CollisionSet{id, top, bottom, right, left}
        }).collect::<Vec<CollisionSet>>();

        FormattedCollision{
            points: p,
            spawns: s,
            collision: col_sets
        }
    }
}

fn condense_to_planes(pi: &[PlaneInfo], con: &[u16]) -> Vec<Planes> {
    // start and length map into the connections array
    // the connections array is an ordered list of indices into collision point array
    // those indices map to points that define the plane
    // create a stand-alone vec of usize indices by slicing off part of the connections array
    pi.iter().map(|&PlaneInfo{start, length}| {
        let end = (start + length) as usize;
        Planes(con[start as usize..end].to_vec())
    }).collect()
}
