mod parts;
mod point;
mod spawn;
mod planeinfo;
pub use self::parts::*;
pub use self::point::CollisionPoint;
pub use self::spawn::Spawn;
pub use self::planeinfo::PlaneInfo;

use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormattedCollision {
    points: BTreeMap<usize, CollisionPoint>,
    collision: Vec<CollisionSet>,
    spawns: Vec<Spawn>
}
#[derive(Debug, Serialize, Deserialize)]
struct CollisionSet {
    id: u16,
    top: Vec<Plane>,
    bottom: Vec<Plane>,
    right: Vec<Plane>,
    left: Vec<Plane>
}
#[derive(Debug, Serialize, Deserialize)]
struct Plane(Vec<u16>);

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

        let points: BTreeMap<usize, CollisionPoint> = p.into_iter().enumerate().map(|(i, val)| (i, val)).collect();

        FormattedCollision{
            points,
            spawns: s,
            collision: col_sets
        }
    }
    pub fn to_parts(&self) -> (Vec<&CollisionPoint>, &[Spawn], Vec<PlaneInfo>, Vec<u16>, Vec<ColDetection>) {
        // return (Vec<&CollisionPoint>, &[Spawn], Vec<PlaneInfo>, Vec<u16>, Vec<ColDetection>)
        let col_points:Vec<&CollisionPoint> = self.points.values().collect();
        let spawn_points = &self.spawns;

        let mut col_dects: Vec<ColDetection> = Vec::with_capacity(self.collision.len());
        let mut point_connections: Vec<u16>  = Vec::new();
        let mut plane_info: Vec<PlaneInfo>   = Vec::new();

        // TODO: make this way less ugly...
        for set in self.collision.iter() {
            let mut detect = [0u16; 9];
            detect[0] = set.id;

            // process top collision planes
            detect[1] = plane_info.len() as u16;
            for plane in set.top.iter() {
                let plane_vec = &plane.0;
                let start = point_connections.len() as u16;
                let len = plane_vec.len() as u16;

                plane_info.push(PlaneInfo::new(start, len));
                for point in plane_vec.iter() { point_connections.push(*point) };
            }
            detect[2] = plane_info.len() as u16 - detect[1];

            // process bottom collision planes
            detect[3] = plane_info.len() as u16;
            for plane in set.bottom.iter() {
                let plane_vec = &plane.0;
                let start = point_connections.len() as u16;
                let len = plane_vec.len() as u16;

                plane_info.push(PlaneInfo::new(start, len));
                for point in plane_vec.iter() { point_connections.push(*point) };
            }
            detect[4] = plane_info.len() as u16 - detect[3];

            // process right collision planes
            detect[5] = plane_info.len() as u16;
            for plane in set.right.iter() {
                let plane_vec = &plane.0;
                let start = point_connections.len() as u16;
                let len = plane_vec.len() as u16;

                plane_info.push(PlaneInfo::new(start, len));
                for point in plane_vec.iter() { point_connections.push(*point) };
            }
            detect[6] = plane_info.len() as u16 - detect[5];

            // process left collision planes
            detect[7] = plane_info.len() as u16;
            for plane in set.left.iter() {
                let plane_vec = &plane.0;
                let start = point_connections.len() as u16;
                let len = plane_vec.len() as u16;

                plane_info.push(PlaneInfo::new(start, len));
                for point in plane_vec.iter() { point_connections.push(*point) };
            }
            detect[8] = plane_info.len() as u16 - detect[7];

            let output = ColDetection::from_raw(&detect);
            col_dects.push(output);
        }

        (col_points, spawn_points, plane_info, point_connections, col_dects)
    }
}

fn condense_to_planes(pi: &[PlaneInfo], con: &[u16]) -> Vec<Plane> {
    // start and length map into the connections array
    // the connections array is an ordered list of indices into collision point array
    // those indices map to points that define the plane
    // create a stand-alone vec of usize indices by slicing off part of the connections array
    pi.iter().map(|&PlaneInfo{start, length}| {
        let end = (start + length) as usize;
        Plane(con[start as usize..end].to_vec())
    }).collect()
}
