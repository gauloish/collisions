pub fn scale(scalar: f32, vector: [f32; 2]) -> [f32; 2] {
    [scalar * vector[0], scalar * vector[1]]
}

pub fn add(first: [f32; 2], second: [f32; 2]) -> [f32; 2] {
    [first[0] + second[0], first[1] + second[1]]
}

pub fn sub(first: [f32; 2], second: [f32; 2]) -> [f32; 2] {
    [first[0] - second[0], first[1] - second[1]]
}

pub fn dot(first: [f32; 2], second: [f32; 2]) -> f32 {
    first[0] * second[0] + first[1] * second[1]
}

pub fn dist(first: [f32; 2], second: [f32; 2]) -> f32 {
    ((first[0] - second[0]).powf(2.0) + (first[1] - second[1]).powf(2.0)).sqrt()
}
