use std::iter::Iterator;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::iter::FromIterator;

fn main() {
    println!("Hello, world!");
    projective_plane_find(2);
    println!("Done world!")
}

// A point in a projective plane, or in a struct which is being built to a projective plane.
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    id: u32
}

// A line in a projective plane, or in a struct which is being built to a projective plane
#[derive(Eq, PartialEq, Debug, Clone)]
struct Line<'a> {
    points: HashSet<&'a Point>
}

impl<'a> Line<'a> {
    fn has_point(&'a self, point: &Point) -> bool {
        self.points.contains(&point)
    }
}

impl<'a> Hash for Line<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // TODO: Maybe improve - might collide too often.
        let mut sum = 0;
        for point in &self.points {
            sum = sum + point.id;
        }
        sum.hash(state);
    }
}

// A candidate for a projective plane.
struct Plane<'a> {
    lines: HashSet<Line<'a>>,
    points: HashSet<Point>,
    order: u32,
}

impl<'a> Plane<'a> {
    // During the entire procedure we wish to preserve the property that there is at most
    // one line connecting any two points.
    //
    // It is not hard to show that, if we have a set system where every line has n + 1 points
    // with n^2 + n + 1 elements and n^2 + n + 1 lines such that any two points have at
    // most one line containing them, then that system is a finite projective plane of order n.
    fn find_all_line_candidates(&'a self) -> HashSet<Line> {
        let mut result = HashSet::new();
        for point in &self.points {
            let mut points_in_line = Vec::with_capacity((self.order + 1) as usize);
            points_in_line.push(point);

            let mut allowed_points: HashSet<&Point> = self.points.iter().collect();
            allowed_points.remove(point);

            for line in &self.lines {
                if line.has_point(point) {
                    for point in &line.points {
                        allowed_points.remove(point);
                    }
                }
            }

            for allowed_point in &allowed_points {
                let mut allowed_points_clone: HashSet<&Point> = allowed_points.clone();
                let mut points_in_line_clone = points_in_line.clone();

                allowed_points_clone.remove(allowed_point);
                points_in_line_clone.push(allowed_point);

                let lines = find_line_candidates(self, points_in_line_clone, allowed_points_clone);

                for line in lines {
                    result.insert(line);
                }
            }
        }

        result
    }
}

fn find_line_candidates<'plane>(plane: &'plane Plane, points_in_line: Vec<&'plane Point>, allowed_points: HashSet<&'plane Point>) -> HashSet<Line<'plane>> {
    let mut allowed_points = allowed_points;

    if points_in_line.len() == (plane.order + 1) as usize {
        let mut set_of_points: HashSet<&'plane Point> = HashSet::from_iter(points_in_line);

        let line = Line { points: set_of_points };

        let mut result = HashSet::with_capacity(1);
        result.insert(line);
        return result;
    }

    let point_to_check = points_in_line.last().unwrap();
    for line in &plane.lines {
        if line.has_point(point_to_check) {
            for point in &line.points {
                allowed_points.remove(point);
            }
        }
    }

    let mut result = HashSet::new();
    for allowed_point in allowed_points.iter() {
        let mut allowed_points_clone = allowed_points.clone();
        let mut points_in_line_clone: Vec<&Point> = points_in_line.clone();

        allowed_points_clone.remove(allowed_point);
        points_in_line_clone.push(allowed_point);

        let lines = find_line_candidates(plane, points_in_line_clone, allowed_points_clone);

        for line in lines {
            result.insert(line);
        }
    }

    result
}

fn new_plane<'a>(order: u32) -> Plane<'a> {
    let cardinality = order * order + order + 1;

    // A finite projective plane of order n has n^2 + n + 1 many points
    let points: HashSet<Point> = (0..cardinality)
        .map(|id| Point { id })
        .collect();

    Plane {
        lines: HashSet::with_capacity(cardinality as usize),
        points,
        order,
    }
}

fn projective_plane_find(order: u32) {
    let plane = new_plane(order);
    let lines = plane.find_all_line_candidates();
    println!("{:?}", lines)
}