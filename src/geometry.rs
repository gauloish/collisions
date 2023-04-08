/// Emulate a point in plane
///
/// * `position`: Point position
#[derive(Clone, Copy, Default, Debug)]
pub struct Point {
    pub position: [f64; 2],
}

/// Emulate a sphere in plane
///
/// * `points`: Sphere points
/// * `center`: Sphere center
/// * `radius`: Sphere radius
#[derive(Clone, Copy)]
pub struct Sphere<const N: usize> {
    pub points: [Point; N],
    pub center: Point,
    pub radius: f64,
}

impl<const N: usize> Sphere<N> {
    /// Create a new sphere
    ///
    /// * `center`: Sphere center
    /// * `radius`: Sphere radius
    pub fn new(point: &[f64; 2], radius: &f64) -> Sphere<N> {
        let center = Point {
            position: point.clone(),
        };

        let points = std::array::from_fn::<_, N, _>(|index| {
            let angle = 2.0 * std::f64::consts::PI * (index as f64) / (N as f64);

            let position: [f64; 2] = [
                center.position[0] + radius * angle.cos(),
                center.position[1] + radius * angle.sin(),
            ];

            Point {
                position: position.clone(),
            }
        });

        Sphere {
            points: points.clone(),
            center: center.clone(),
            radius: radius.clone(),
        }
    }
}
