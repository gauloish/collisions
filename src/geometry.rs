use crate::settings;
use rand;

/// Emulate a point in plane
///
/// * `position`: Point position
/// * `color`: Point color
#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub position: [f64; 2],
    pub color: [f64; 3],
}

/// Emulate a sphere in plane
///
/// * `points`: Sphere points
/// * `center`: Sphere center (using center as uniform in render)
/// * `velocity`: Sphere velocity (using velocity as uniform in render)
/// * `radius`: Sphere radius
#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub points: [Point; settings::LENGTH],
    pub center: [f64; 2],
    pub velocity: [f64; 2],
    pub radius: f64,
}

impl Point {
    /// Create a point from array
    ///
    /// * `position`: Point position
    pub fn new(position: [f64; 2]) -> Point {
        let index = rand::random::<usize>() % 7;
        let color = settings::COLORS[index];

        Point { position, color }
    }
}

impl Default for Point {
    /// Default values
    fn default() -> Self {
        Point {
            position: [0.0, 0.0],
            color: [0.0, 0.0, 0.0],
        }
    }
}

impl Sphere {
    /// Create a new sphere
    ///
    /// * `center`: Sphere center
    /// * `radius`: Sphere radius
    pub fn new(center: [f64; 2], radius: f64) -> Sphere {
        let points: [Point; settings::LENGTH] = std::array::from_fn(|index| {
            let angle = 2.0 * std::f64::consts::PI * (index as f64) / (settings::LENGTH as f64);

            let position: [f64; 2] = [
                center[0] + radius * angle.cos(),
                center[1] + radius * angle.sin(),
            ];

            let color = rand::random::<usize>() % 7;

            Point {
                position,
                color: settings::COLORS[color],
            }
        });

        let angle = rand::random::<f64>();
        let multiplier = 1 + rand::random::<usize>() % 100;

        let velocity: [f64; 2] = [
            ((multiplier as f64) / 100.0) * radius * angle.cos(),
            ((multiplier as f64) / 100.0) * radius * angle.sin(),
        ];

        Sphere {
            points,
            center,
            velocity,
            radius,
        }
    }
}
