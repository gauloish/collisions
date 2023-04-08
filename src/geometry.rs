use crate::settings;

/// Emulate a point in plane
///
/// * `position`: Point position
#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub position: [f64; 2],
}

/// Emulate a sphere in plane
///
/// * `points`: Sphere points
/// * `center`: Sphere center
/// * `radius`: Sphere radius
#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub points: [Point; settings::LENGTH],
    pub center: Point,
    pub radius: f64,
}

impl Point {
    /// Create a point from array
    ///
    /// * `point`: Array
    pub fn from(position: [f64; 2]) -> Point {
        Point { position }
    }
}

impl Sphere {
    /// Create a new sphere
    ///
    /// * `center`: Sphere center
    /// * `radius`: Sphere radius
    pub fn new(center: Point, radius: f64) -> Sphere {
        let points: [Point; settings::LENGTH] = std::array::from_fn(|index| {
            let angle = 2.0 * std::f64::consts::PI * (index as f64) / (settings::LENGTH as f64);

            let position: [f64; 2] = [
                center.position[0] + radius * angle.cos(),
                center.position[1] + radius * angle.sin(),
            ];

            Point { position }
        });

        Sphere {
            points,
            center,
            radius,
        }
    }
}
