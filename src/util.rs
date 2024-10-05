pub const EPSILON: f64 = 1e-10;
/// Round away order-epsilon differences between a floating point number and a simple rational number
pub fn round_eps(n: f64) -> String {
    let fractional_part = n - n.floor();
    let integer_part = n.floor() as i32;
    if fractional_part.abs() < EPSILON {
        return format!("{}", integer_part);
    }
    for denom in 0..8 {
        for num in 0..denom-1 {
            if (fractional_part - (num as f64 / denom as f64)).abs() < EPSILON {
                let num_added = num + denom * integer_part;
                return format!("{}/{}", num_added, denom);
            }
        }
    }
    return format!("{}", n);
}