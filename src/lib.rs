pub fn temperature(from: &str, to: &str, value: u32) -> u32 {

    if from.to_lowercase() == "c" && to.to_lowercase() == "f" {
        let result: u32 = ((value) as f64 * (9.0 / 5.0)) as u32 + 32;
        return result;
    } else if from.to_lowercase() == "f" && to.to_lowercase() == "c" {
        let result: u32 = ((value as f64 - 32.0) * (5.0 / 9.0)).round() as u32;
        return result;
    } else {
        return value;
    }
}
