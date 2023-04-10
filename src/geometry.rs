use crate::settings;
use rand;

/// Emulate a point in plane
///
/// * `position`: Point position
/// * `color`: Point color
#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub position: [f32; 2],
    pub color: [f32; 3],
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
    pub center: [f32; 2],
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Point {
    /// Create a point from array
    ///
    /// * `position`: Point position
    pub fn new(position: [f32; 2]) -> Point {
        let index = rand::random::<usize>() % 7;
        let color = settings::COLORS[index];

        Point { position, color }
    }
}

impl Sphere {
    /// Create a new sphere
    ///
    /// * `center`: Sphere center
    /// * `radius`: Sphere radius
    pub fn new(center: [f32; 2], radius: f32) -> Sphere {
        let color = rand::random::<usize>() % 7;

        let points: [Point; settings::LENGTH] = std::array::from_fn(|index| {
            let angle = 2.0 * std::f32::consts::PI * (index as f32) / (settings::LENGTH as f32);

            let position: [f32; 2] = [
                radius * angle.cos(), //
                radius * angle.sin(), //
            ];

            Point {
                position,
                color: settings::COLORS[color],
            }
        });

        let angle = rand::random::<f32>();
        let multiplier = 1 + rand::random::<usize>() % 100;

        let velocity: [f32; 2] = [
            ((multiplier as f32) / 100.0) * radius * angle.cos(),
            ((multiplier as f32) / 100.0) * radius * angle.sin(),
        ];

        Sphere {
            points,
            center,
            velocity,
            radius,
        }
    }
}
