/// Emulate a sphere in plane
///
/// * `points`: Sphere points
/// * `center`: Sphere center
/// * `radius`: Sphere radius
pub struct Sphere<const N: usize> {
    pub points: [[f64; 2]; N],
    pub center: [f64; 2],
    pub radius: f64,
}

impl<const N: usize> Sphere<N> {
    /// Create a new sphere
    ///
    /// * `center`: Sphere center
    /// * `radius`: Sphere radius
    pub fn new(center: &[f64; 2], radius: &f64) -> Sphere<N> {
        let points = std::array::from_fn::<_, N, _>(|index| {
            let angle = 2.0 * std::f64::consts::PI * (index as f64) / (N as f64);

            [
                center[0] + radius * angle.cos(),
                center[1] + radius * angle.sin(),
            ]
        });

        Sphere {
            points: points.clone(),
            center: center.clone(),
            radius: radius.clone(),
        }
    }
}
