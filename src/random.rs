use js_sys::Math;

pub fn random() -> f64 {
    Math::random()
}

pub fn random_range_i32(from: i32, to: i32) -> i32 {
    let factor = Math::random();

    let range = to - from;
    let number = factor * (range as f64);

    from + (number as i32)
}
