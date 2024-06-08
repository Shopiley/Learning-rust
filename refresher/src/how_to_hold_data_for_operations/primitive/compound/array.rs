
pub fn multiplier(numbers: &[f64]) -> f64{
    let mut result = 1f64;

    for &nums in numbers {
        result *= nums;
    }
    return result
}